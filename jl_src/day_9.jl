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
function ProcessFile{T}(filepath::String)::Vector{Vector{T}} where T
    result = Vector{Vector{T}}()
    open(filepath) do f
        for line in readlines(f)
            new_line = Vector{T}()
            for char in line
                push!(new_line, parse(T, char))
            end
            push!(result, new_line)
        end
    end
    result
end

function SolveProb1(the_map::Vector{Vector{T}}) where T <: Number
    height = length(the_map)
    width = length(the_map[1])

    function check_bounds(i::T,j::T)::Bool where T <: Number
        if i < 1 || i > height || j < 1 || j > width
            return false
        else
            return true
        end
    end

    total_risk = 0
    for i in 1:height
        for j in 1:width
            val = the_map[i][j]
            neighbors = Vector{T}()
            if check_bounds(i+1,j)
                push!(neighbors, the_map[i+1][j])
            end
            if check_bounds(i-1,j)
                push!(neighbors, the_map[i-1][j])
            end
            if check_bounds(i,j+1)
                push!(neighbors, the_map[i][j+1])
            end
            if check_bounds(i,j-1)
                push!(neighbors, the_map[i][j-1])
            end
            if all(map(v -> v > val, neighbors))
                total_risk += val+1
            end
        end
    end
    println("Day 9 problem 1: ", total_risk)
end

# Get input filepath
input_filepath = parsed_args["input"]
verbose = parsed_args["verbose"]

the_map = ProcessFile{Int}(input_filepath)

SolveProb1(the_map)
