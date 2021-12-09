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

Charset = Set{Char}

# Process files
function ProcessFile(filepath::String)::Vector{Tuple{Vector{Charset},Vector{Charset}}}
    results = Vector{Tuple{Vector{Charset},Vector{Charset}}}()
    open(filepath) do f
        for line in readlines(f)
            line_parts = split(line, '|')
            function splitter(s)
                collect(map(s -> Set(strip(s)), split(strip(s))))
            end
            unique_patterns = splitter(line_parts[1])
            messages = splitter(line_parts[2])
            push!(results, (unique_patterns, messages))
        end
    end
    results
end

# Get input filepath
input_filepath = parsed_args["input"]
verbose = parsed_args["verbose"]

# Define true seg map
true_num_seg_map = Dict{Int,Charset}([
                                        (0, Charset(['a', 'b', 'c', 'e', 'f', 'g'])),
                                        (1, Charset(['c', 'f'])),
                                        (2, Charset(['a', 'c', 'd', 'e', 'g'])),
                                        (3, Charset(['a', 'c', 'd', 'f', 'g'])),
                                        (4, Charset(['b', 'c', 'd', 'f'])),
                                        (5, Charset(['a', 'b', 'd', 'f', 'g'])),
                                        (6, Charset(['a', 'b', 'd', 'e', 'f', 'g'])),
                                        (7, Charset(['a', 'c', 'f'])),
                                        (8, Charset(['a', 'b', 'c', 'd', 'e', 'f', 'g'])),
                                        (9, Charset(['a', 'b', 'c', 'd', 'f', 'g'])),
                                    ])

function decode(code::Tuple{Vector{Charset},Vector{Charset}})::String
    code_map = Dict{Charset,Int}()
    map_code = Dict{Int,Charset}()
    # Map between code segments and real segments
    seg_map = Dict{Char,Char}()
    # Categorize unique patterns by number of segments
    num_seg_map = Dict{Int,Vector{Charset}}()

    for pattern in code[1]
        num_segs = length(pattern)
        map_vec = get!(num_seg_map, num_segs, Vector{Charset}())
        push!(map_vec, pattern)
    end

    # Get unique intersection patterns
    seg5_pat = intersect(get(num_seg_map, 5, Vector{Charset}())...)
    seg6_pat = intersect(get(num_seg_map, 6, Vector{Charset}())...)

    # Store known unique patterns
    pat = get(num_seg_map, 2, Vector{Charset}())[1]
    code_map[pat] = 1
    map_code[1] = pat

    pat = get(num_seg_map, 3, Vector{Charset}())[1]
    code_map[pat] = 7
    map_code[7] = pat

    pat = get(num_seg_map, 4, Vector{Charset}())[1]
    code_map[pat] = 4
    map_code[4] = pat

    pat = get(num_seg_map, 7, Vector{Charset}())[1]
    code_map[pat] = 8
    map_code[8] = pat

    # Compute segs

    a_seg = collect(intersect(map_code[7], seg5_pat))[1]
    seg_map['a'] = a_seg

    d_seg = collect(intersect(map_code[4], seg5_pat))[1]
    seg_map['d'] = d_seg

    g_seg = collect(setdiff(seg5_pat, Charset([a_seg, d_seg])))[1]
    seg_map['g'] = g_seg

    f_seg = collect(intersect(seg6_pat, map_code[1]))[1]
    seg_map['f'] = f_seg

    c_seg = collect(setdiff(map_code[1], Charset([f_seg])))[1]
    seg_map['c'] = c_seg

    b_seg = collect(setdiff(map_code[4], Charset([c_seg, f_seg, d_seg])))[1]
    seg_map['b'] = b_seg

    e_seg = collect(setdiff(map_code[8], Charset([a_seg, d_seg, g_seg, f_seg, c_seg, b_seg])))[1]
    seg_map['e'] = e_seg

    # Fill out code map
    for (num, true_seg) in true_num_seg_map
        coded_seg = Charset()

        for seg in true_seg
            push!(coded_seg, seg_map[seg])
        end
        code_map[coded_seg] = num
    end

    string(map(n -> string(code_map[n]), code[2])...)
end

codes = ProcessFile(input_filepath)

messages = collect(map(c -> decode(c), codes))

function solve(msgs::Vector{String})
    num_easy = 0
    num = 0

    for msg in messages
        num_easy += length(filter(c -> c in ['1', '4', '7', '8'], msg))
        num += parse(Int, msg)
    end

    println("Day 8 problem 1: ", num_easy)
    println("Day 8 problem 2: ", num)
end

solve(messages)
