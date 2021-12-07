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

struct Point{T}
    x::T
    y::T
end

function Base.isequal(a::Point{T}, b::Point{T}) where T
    (a.x == b.x) && (a.y == b.y)
end

function Base.hash(a::Point{T}, h::UInt) where T
    #hash(a.y, hash(a.x, hash(:Point{T}, h)))
    hash((a.x, a.y))
end

struct ProcessFile{T} end

# Process files
function ProcessFile{T}(filepath::String)::Vector{Tuple{Point{T},Point{T}}} where T
    open(filepath) do f
        result = Vector{Tuple{Point{T},Point{T}}}()
        for line in readlines(f)
            res = match(r"([0-9]*),([0-9]*) -> ([0-9]*),([0-9]*)", line)
            left = Point{T}(parse(T, res.captures[1]), parse(T, res.captures[2]))
            right = Point{T}(parse(T, res.captures[3]), parse(T, res.captures[4]))
            push!(result, (left, right))
        end
        return result
    end
end

function HorizOrVertLine(left::Point{T}, right::Point{T}) where T
    result = Vector{Point{T}}()
    if left.x == right.x
        x = left.x
        min_y = min(left.y, right.y)
        max_y = max(left.y, right.y)
        for y in min_y:max_y
            push!(result, Point{T}(x,y))
        end
    elseif left.y == right.y
        y = left.y
        min_x = min(left.x, right.x)
        max_x = max(left.x, right.x)
        for x in min_x:max_x
            push!(result, Point{T}(x,y))
        end
    end
    return result
end

function HorizVertOrDiagLine(left::Point{T}, right::Point{T}) where T
    result = Vector{Point{T}}()
    if left.x == right.x
        # Horizontal Line
        x = left.x
        min_y = min(left.y, right.y)
        max_y = max(left.y, right.y)
        for y in min_y:max_y
            push!(result, Point{T}(x,y))
        end
    elseif left.y == right.y
        # Vertical Line
        y = left.y
        min_x = min(left.x, right.x)
        max_x = max(left.x, right.x)
        for x in min_x:max_x
            push!(result, Point{T}(x,y))
        end
    else
        # Diagonal Line
        x_diff = right.x-left.x
        y_diff = right.y-left.y
        num = abs(x_diff)
        if abs(y_diff) != num
            throw(InvalidStateException("found line which isn't diagonal"))
        end
        diffs = collect(0:num)
        if left.x < right.x
            x_diffs = diffs
        else
            x_diffs = -diffs
        end
        if left.y < right.y
            y_diffs = diffs
        else
            y_diffs = -diffs
        end
        all_diffs = zip(x_diffs, y_diffs)
        point_from_diff = function (d)
            Point{T}(left.x+d[1], left.y+d[2])
        end
        points = map(point_from_diff, all_diffs)
        for p in points
            push!(result, p)
        end
    end
    return result
end

function SolveProblem1(line_ends::Vector{Tuple{Point{T},Point{T}}}) where T
    horiz_vert_selector = function (t)
        (t[1].x == t[2].x) || (t[1].y == t[2].y)
    end
    line_ends = collect(filter(horiz_vert_selector, line_ends))

    position_count = Dict{Point{T}, UInt}()

    add_to_count = function(p::Point{T})
        position_count[p] = get(position_count, p, 0)+1
    end

    num_added = 0
    for line in line_ends
        points_to_add = HorizOrVertLine(line[1], line[2])
        for p in points_to_add
            add_to_count(p)
            num_added += 1
        end
    end

    num = 0
    for v in values(position_count)
        if v > 1
            num += 1
        end
    end

    println("Day 5 Problem 1: ", num)
end

function SolveProblem2(line_ends::Vector{Tuple{Point{T},Point{T}}}) where T
    position_count = Dict{Point{T}, UInt}()

    add_to_count = function(p::Point{T})
        position_count[p] = get(position_count, p, 0)+1
    end

    num_added = 0
    for line in line_ends
        points_to_add = HorizVertOrDiagLine(line[1], line[2])
        for p in points_to_add
            add_to_count(p)
            num_added += 1
        end
    end

    num = 0
    for v in values(position_count)
        if v > 1
            num += 1
        end
    end

    println("Day 5 Problem 2: ", num)
end

# Get input filepath
input_filepath = parsed_args["input"]
verbose = parsed_args["verbose"]

line_ends = ProcessFile{Int}(input_filepath)

SolveProblem1(line_ends)
SolveProblem2(line_ends)
