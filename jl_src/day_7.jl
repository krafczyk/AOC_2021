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

function fuel_cost1(pos)
    sum(map(p -> abs(p-pos), positions))
end

min_fuel1 = minimum(collect(map(p -> fuel_cost1(p), possible_positions)))

println("Day 7 problem 1: ", min_fuel1)

function fuel_cost(d::T)::T where T
    d*(d+1)/2
end

function fuel_cost2(pos)
    sum(map(p -> fuel_cost(abs(p-pos)), positions))
end

min_fuel2 = minimum(collect(map(p -> fuel_cost2(p), possible_positions)))

println("Day 7 problem 2: ", min_fuel2)
