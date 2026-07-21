#!/usr/bin/env ruby
# frozen_string_literal: true

require "fileutils"
require "io/console"
require "json"
require "time"

def default_cookie_path
  candidates = []
  home = Dir.home

  if /darwin/ =~ RUBY_PLATFORM
    candidates << File.join(home, "Library", "Application Support", "cargo-compete", "cookies.jsonl")
  end

  xdg_data_home = ENV.fetch("XDG_DATA_HOME", File.join(home, ".local", "share"))
  candidates << File.join(xdg_data_home, "cargo-compete", "cookies.jsonl")
  candidates << File.join(home, ".local", "share", "cargo-compete", "cookies.jsonl")

  candidates.find { |path| File.exist?(path) } || candidates.first
end

def read_session
  raw = ENV["REVEL_SESSION"]

  if raw.nil? || raw.empty?
    if STDIN.tty?
      print "REVEL_SESSION: "
      raw = STDIN.noecho(&:gets)
      puts
    else
      raw = STDIN.read
    end
  end

  session = raw.to_s.strip
  match = session.match(/(?:^|[;\s])REVEL_SESSION=([^;\s]+)/)
  session = match[1] if match

  if session.empty?
    warn "REVEL_SESSION is empty"
    exit 1
  end

  if session.include?("\n") || session.include?("\r") || session.include?(";")
    warn "REVEL_SESSION must be a single cookie value"
    exit 1
  end

  session
end

def replacement_cookie(raw_cookie, session)
  if raw_cookie.start_with?("REVEL_SESSION=")
    raw_cookie.sub(/\AREVEL_SESSION=[^;]*/, "REVEL_SESSION=#{session}")
  else
    raw_cookie
  end
end

cookie_path = ENV.fetch("CARGO_COMPETE_COOKIES", default_cookie_path)
session = read_session

entries =
  if File.exist?(cookie_path)
    File.readlines(cookie_path, chomp: true).reject(&:empty?).map { |line| JSON.parse(line) }
  else
    []
  end

updated = false
entries.each do |entry|
  raw_cookie = entry["raw_cookie"]
  next unless raw_cookie.is_a?(String) && raw_cookie.start_with?("REVEL_SESSION=")

  entry["raw_cookie"] = replacement_cookie(raw_cookie, session)
  updated = true
end

unless updated
  expires_at = (Time.now.utc + 180 * 24 * 60 * 60).iso8601
  entries << {
    "raw_cookie" => "REVEL_SESSION=#{session}; HttpOnly; SameSite=Lax",
    "path" => ["/", true],
    "domain" => { "HostOnly" => "atcoder.jp" },
    "expires" => { "AtUtc" => expires_at }
  }
end

FileUtils.mkdir_p(File.dirname(cookie_path))
tmp_path = "#{cookie_path}.tmp"
File.open(tmp_path, "w", 0o600) do |file|
  entries.each { |entry| file.puts(JSON.generate(entry)) }
end
File.rename(tmp_path, cookie_path)
File.chmod(0o600, cookie_path)

puts "Updated #{cookie_path}"
