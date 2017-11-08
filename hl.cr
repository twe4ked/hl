require "option_parser"

COLORS = {
  "red" => 31,
  "green" => 32,
  "yellow" => 33,
  "blue" => 34,
  "magenta" => 35,
  "cyan" => 36,
  "white" => 37,
}
RESET_COLOR = "\e[0m"

def color_code(name); "\e[0;#{COLORS[name]}m"; end

color_order = 0
patterns = Hash(String, Tuple(String, Int32)).new
begin
  parser = OptionParser.parse! do |parser|
    parser.banner = "Usage: hl [--COLOR=PATTERN ...]"
    COLORS.each do |color, _|
      parser.on("-#{color.chars[0]} PATTERN", "--#{color}=PATTERN", "Highlight PATTERN in #{color}") do |pattern|
        patterns[color] = {pattern, color_order}
        color_order += 1
      end
    end
    parser.on("-h", "--help", "Display help") { puts parser }
  end
rescue OptionParser::InvalidOption
  puts "Unknown option."
  exit 1
end
if patterns.empty?
  puts parser
  exit
end

content = STDIN.gets_to_end

indices = Hash(Int32, Array(Tuple(String, String))).new([] of Tuple(String, String))
patterns.each do |color, (pattern, _)|
  content.scan(/#{pattern}/m).each do |match_data|
    indices[match_data.begin.as(Int32)] += [{color, "begin"}]
    indices[match_data.end.as(Int32)] += [{color, "end"}]
  end
end

stack = [] of String
content.each_char.with_index do |char, i|
  indices[i].sort { |(color, _)| 0 - patterns[color][1] }.each do |color, begin_or_end|
    if begin_or_end == "begin"
      stack << color.as(String)
      print color_code(color)
    else
      stack.delete(color)
      print RESET_COLOR + stack.map { |color| color_code(color) }.join
    end
  end
  print char
end
