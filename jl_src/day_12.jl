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
function ProcessFile(filepath::String)::Dict{String,Set{String}}
    result_map = Dict{String,Set{String}}();
    open(filepath) do f
        for line in readlines(f)
            pair = split(line, '-')
            push!(get!(result_map, pair[1], Set{String}()), pair[2])
            push!(get!(result_map, pair[2], Set{String}()), pair[1])
        end
    end
    result_map
end

function num_paths1(cur_path::Vector{String}, cave_map::Dict{String,Set{String}})::UInt
    if cur_path[end] == "end"
        # We made it to the end. this is a valid path.
        return 1
    end

    # Get room list of last room in the path
    possible_next_steps = cave_map[cur_path[end]]
    visited = filter(s -> lowercase(s) == s, cur_path)

    possible_next_steps = setdiff(possible_next_steps, visited)

    number_of_paths::UInt = 0

    for step in collect(possible_next_steps)
        new_path = copy(cur_path)
        push!(new_path, step)
        number_of_paths += num_paths1(new_path, cave_map)
    end

    number_of_paths
end

function num_paths2(cur_path::Vector{String}, cave_map::Dict{String,Set{String}})::UInt
    if cur_path[end] == "end"
        # We made it to the end. this is a valid path.
        return 1
    end

    is_lowercase = (s -> lowercase(s) == s)

    #println!("cur_path: {:?}", cur_path);
    # Get room list of last room in the path
    possible_next_steps = cave_map[cur_path[end]]
    # Get list of rooms we can no longer visit. (always start with start)
    cant_revisit = Set{String}([cur_path[1]])
    # Determine if we've visited a small cave twise
    small_caves_visited = collect(filter(is_lowercase, cur_path))
    cave_count = Set{String}();
    visited_small_cave_twice::Bool = false
    for cave in small_caves_visited
        if ! (cave in cave_count)
            push!(cave_count,cave)
        else
            visited_small_cave_twice = true
        end
    end

    if visited_small_cave_twice
        small_caves_visited = collect(filter(is_lowercase, cur_path))
        for cave in small_caves_visited
            push!(cant_revisit,cave)
        end
    end

    possible_next_steps = collect(setdiff(possible_next_steps, cant_revisit))
#    //println!("possible_next_steps: {:?}", possible_next_steps);
#
    number_of_paths::UInt = 0

    for step in collect(possible_next_steps)
        new_path = copy(cur_path)
        push!(new_path, step)
        number_of_paths += num_paths2(new_path, cave_map)
    end

    number_of_paths
end

# Get input filepath
input_filepath = parsed_args["input"]
verbose = parsed_args["verbose"]

cave_map = ProcessFile(input_filepath)

println("Day 12 problem 1: ", num_paths1(Vector{String}(["start"]), cave_map))
println("Day 12 problem 2: ", num_paths2(Vector{String}(["start"]), cave_map))
