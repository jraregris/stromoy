#!/usr/bin/env rake
require 'bundler/gem_tasks'
require 'cucumber/rake/task'
require 'rspec/core/rake_task'

Cucumber::Rake::Task.new
RSpec::Core::RakeTask.new


task :default => :test
task :test => [:cucumber, :spec]
