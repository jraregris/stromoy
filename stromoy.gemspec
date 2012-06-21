# -*- encoding: utf-8 -*-
require File.expand_path('../lib/stromoy/version', __FILE__)

Gem::Specification.new do |gem|
  gem.authors       = ["Oddmund"]
  gem.email         = ["oddmund@oddmundo.com"]
  gem.description   = %q{A simple relative searcher.}
  gem.summary       = %q{A command line tool that lets you search for relative byte sequences in binary files.}
  gem.homepage      = ""

  gem.files         = `git ls-files`.split($\)
  gem.executables   = gem.files.grep(%r{^bin/}).map{ |f| File.basename(f) }
  gem.test_files    = gem.files.grep(%r{^(test|spec|features)/})
  gem.name          = "stromoy"
  gem.require_paths = ["lib"]
  gem.version       = Stromoy::VERSION
  
  gem.add_dependency 'slop'

  gem.add_development_dependency 'rspec'
  gem.add_development_dependency 'cucumber'
  gem.add_development_dependency 'aruba'
end
