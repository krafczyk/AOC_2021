import ArgParse
import Pkg
Pkg.add("DataStructures");
import DataStructures

# Build ArgParsing Table
s = ArgParse.ArgParseSettings()
@ArgParse.add_arg_table s begin
    "--input"
        help = "Filepath to input data"
        arg_type = String
        required = true
    "--verbose"
        help = "Add additional reporting"
        action = :store_true
end

# Parse arguments
parsed_args = ArgParse.parse_args(ARGS, s)

# Process files
function ProcessFile(filepath::String)::Vector{String}
    results = Vector{String}()
    open(filepath) do f
        for line in readlines(f)
            push!(results, line)
        end
    end
    results
end

abstract type Block end
struct Regular <: Block end
struct Square <: Block end
struct Curly <: Block end
struct Angle <: Block end
struct None <: Block end

char_map = Dict{Char,Block}([ 
    ('(', Regular()),
    (')', Regular()),
    ('[', Square()),
    (']', Square()),
    ('{', Curly()),
    ('}', Curly()),
    ('<', Angle()),
    ('>', Angle())]);

function get_illegal_char(line::String)::Block
    block_stack = DataStructures.Deque{Block}();
    for ch in line
        if ch in ['(', '[', '{', '<']
            cur_block = char_map[ch]
            pushfirst!(block_stack, cur_block)
        elseif ch in [')', ']', '}', '>' ]
            cur_block = char_map[ch]
            last_block = popfirst!(block_stack)
            if cur_block != last_block
                return cur_block
            end
        else
            throw(InvalidStateException("Encountered unexpected character! ", ch));
        end
    end
    None()
end

function get_line_completion(line::String)::Vector{Block}
    result = Vector{Block}()
    block_stack = DataStructures.Deque{Block}();
    for ch in line
        if ch in ['(', '[', '{', '<']
            cur_block = char_map[ch]
            pushfirst!(block_stack, cur_block)
        elseif ch in [')', ']', '}', '>' ]
            cur_block = char_map[ch]
            last_block = popfirst!(block_stack)
            if cur_block != last_block
                return Vector{Block}([None()])
            end
        else
            throw(InvalidStateException("Encountered unexpected character! ", ch));
        end
    end
    for b in block_stack
        push!(result, b)
    end
    result
end

function score_line(blocks::Vector{Block})::UInt
    add_map = Dict{Block,UInt}([
        (Regular(), 1),
        (Square(), 2),
        (Curly(), 3),
        (Angle(), 4)
    ]);

    result = 0;
    for b in blocks
        result = result*5+add_map[b]
    end
    result
end

# Get input filepath
input_filepath = parsed_args["input"]
verbose = parsed_args["verbose"]

nav_data = ProcessFile(input_filepath)

score_map = Dict{Block,UInt}([
    (Regular(), 3),
    (Square(), 57),
    (Curly(), 1197),
    (Angle(), 25137)
])

score_blocks = filter(r -> r != None(), map(s -> get_illegal_char(s), nav_data))
syntax_error_score = sum(map(b -> score_map[b], score_blocks))
println("Day 10 problem 1: ", syntax_error_score)

function v_test(v::Vector{Block})::Bool
    if length(v) == 1
        if v[1] == None()
            return false
        else
            return true
        end
    else
        return true
    end
end

line_blocks = filter(r -> v_test(r), map(s -> get_line_completion(s), nav_data))
line_scores = collect(map(v -> score_line(v), line_blocks))

sort!(line_scores)

middle_score = line_scores[(length(line_scores)÷2)+1];

println("Day 10 problem 2: ", middle_score);

