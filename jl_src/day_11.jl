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
function ProcessFile{T}(filepath::String)::Array{T,2} where T <: Number
    result = Vector{Vector{T}}()
    open(filepath) do f
        for line in readlines(f)
            num_line = Vector{T}()
            for ch in line
                push!(num_line, parse(T,ch))
            end
            push!(result, num_line)
        end
    end
    height = length(result)
    width = length(result[1])
    res_map = zeros(T, height, width)
    for i in 1:height
        for j in 1:width
            res_map[i,j] = result[i][j]
        end
    end
    res_map
end

struct Point
    i::UInt
    j::UInt
end

function do_step(the_map::Array{T,2})::Tuple{Array{T,2},UInt} where T
    function check_bounds(p::Point)::Bool
        !(p.i < 1 || p.i > size(the_map)[1] || p.j < 1 || p.j > size(the_map)[2])
    end
    function get_neighbors(p::Point)::Vector{Point}
        i = p.i
        j = p.j
        ts = [(i+1,j+1),(i,j+1),(i-1,j+1),(i+1,j),(i-1,j),(i+1,j-1),(i,j-1),(i-1,j-1)]
        ns = map(t -> Point(t[1], t[2]), ts)
        return collect(filter(n -> check_bounds(n), ns))
    end

    # First, increment all by one
    the_map = the_map.+1

    # Second loop while there are flashers
    num_flashes = 0
    while any(the_map.>9)
        for i in 1:size(the_map)[1]
            for j in 1:size(the_map)[2]
                p = Point(i,j)
                if the_map[i,j] > 9
                    # We have one ready to flash
                    neighbors = get_neighbors(p)
                    for n in neighbors
                        val = the_map[n.i,n.j]
                        if val != 0
                            the_map[n.i,n.j] += 1
                        end
                    end
                    # Set the point to zero
                    the_map[i,j] = 0
                    num_flashes += 1
                end
            end
        end
    end
    (the_map, num_flashes)
end

function SolveProblem(the_map::Array{T,2}) where T
    flash_sequence = Vector{UInt}()
    num_steps = 100
    total_flashes = 0

    while !all(the_map.== 0)
        the_map, flashes = do_step(the_map)
        push!(flash_sequence, flashes)
    end

    println("Day 11 problem 1: ", sum(flash_sequence[1:100]))
    println("Day 11 problem 2: ", length(flash_sequence))
end

# Get input filepath
input_filepath = parsed_args["input"]
verbose = parsed_args["verbose"]

the_map = ProcessFile{Int}(input_filepath)

SolveProblem(the_map)
