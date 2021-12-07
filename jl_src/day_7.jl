import ArgParse

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
struct ProcessFile{T} end
function ProcessFile{T}(filepath::String)::Vector{T} where T
    open(filepath) do f
        for line in readlines(f)
            return collect(map(s -> parse(T, s), split(line, ',')))
        end
    end
end

# Get input filepath
input_filepath = parsed_args["input"]
verbose = parsed_args["verbose"]

positions = ProcessFile{Int}(input_filepath)

min_pos = minimum(positions)
max_pos = maximum(positions)

possible_positions = collect(min_pos:max_pos)

function fuel_cost(pos)
    sum(map(p -> abs(p-pos), positions))
end

min_fuel = minimum(collect(map(p -> fuel_cost(p), possible_positions)))

println("Day 7 problem 1: ", min_fuel)
