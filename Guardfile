require 'guard/plugin'

module ::Guard
  class Make < ::Guard::Plugin
    def run_all
      system('make', 'clean')
      system('make', 'check')
    end

    def run_on_changes(*)
      system('make', 'check')
    end
  end
end

guard(:make) do
  watch(%r{src/.*\.rs})
  watch(%r{test/.*\.rs})
end

# vim: ft=ruby
