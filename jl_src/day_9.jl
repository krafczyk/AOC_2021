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

Point{T} = Tuple{T,T} where T

function SolveProb(the_map::Vector{Vector{T}}) where T <: Number
    height = length(the_map)
    width = length(the_map[1])

    function check_bounds(p::Point{T})::Bool
        i = p[1]
        j = p[2]
        if i < 1 || i > height || j < 1 || j > width
            return false
        else
            return true
        end
    end

    function get_neighbors(p::Point{T})::Set{Point{T}}
        i = p[1]
        j = p[2]
        neighbors = Set{Point{T}}()
        function add_neighbor(p::Point{T})
            if check_bounds(p)
                push!(neighbors, p)
            end
        end
        for p in [(i+1,j), (i-1,j), (i,j+1), (i,j-1)]
            add_neighbor(p)
        end
        neighbors
    end

    function get_val(p::Point{T})::T
        return the_map[p[1]][p[2]]
    end

    basins = Vector{Point{T}}()

    total_risk = 0
    for i in 1:height
        for j in 1:width
            val = the_map[i][j]
            neighbors = get_neighbors((i,j))
            if all(map(p -> get_val(p) > val, collect(neighbors)))
                # This is a basin. push it onto the list.
                push!(basins, (i,j))
                total_risk += val+1
            end
        end
    end
    println("Day 9 problem 1: ", total_risk)

    # Calculate basin size. Here I'm using the apparent trick, that the edges of each basin are marked by tiles of amount 9.
    basin_maps = Dict{Point{T},Set{Point{T}}}()
    for basin_point in basins
        # initialize visit list
        visit = Set{Point{T}}([basin_point])
        visited = Set{Point{T}}()
        basin_elements = Set{Point{T}}()
        while length(visit) != 0
            # pop point from visit list
            point = pop!(visit)
            # Get point neighbors
            neighbors = get_neighbors(point)
            # remove already visited points
            setdiff!(neighbors, visited)
            for neighbor in neighbors
                val = get_val(neighbor)
                if val == 9
                    # found edge point
                    push!(visited, neighbor)
                else
                    # found point to add to visit
                    push!(visit, neighbor)
                end
            end
            # Add point to the basin elements
            push!(basin_elements, point)
            push!(visited, point)
        end
        basin_maps[basin_point] = basin_elements
    end

    largest_basins_prod = last(cumprod(reverse(sort(map(s -> length(s), values(basin_maps))))[1:3]))
    println("Day 9 problem 2: ", largest_basins_prod)
end

# Get input filepath
input_filepath = parsed_args["input"]
verbose = parsed_args["verbose"]

the_map = ProcessFile{Int}(input_filepath)

SolveProb(the_map)
