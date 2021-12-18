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

abstract type Fold{T} end
struct FoldX{T} <: Fold{T}
    pos::T
end
struct FoldY{T} <: Fold{T}
    pos::T
end

# Parse arguments
parsed_args = ArgParse.parse_args(ARGS, s)

# Process files
struct ProcessFile{T} end
function ProcessFile{T}(filepath::String)::Tuple{Set{Tuple{T,T}},Vector{Fold{T}}} where T
    points = Set{Tuple{T,T}}()
    folds = Vector{Fold{T}}()
    open(filepath) do f
        for line in readlines(f)
            if line == ""
                continue
            end
            line_split = split(line)
            if line_split[1] == "fold"
                # We have a fold instruction
                fold_bits = split(line_split[3],"=")
                val = parse(T, fold_bits[2])
                if fold_bits[1] == "y"
                    push!(folds, FoldY{T}(val))
                else
                    push!(folds, FoldX{T}(val))
                end
            else
                num_bits = split(line, ',')
                push!(points, (parse(T,num_bits[1]), parse(T,num_bits[2])))
            end
        end
    end
    (points, folds)
end

function FoldPoints(points::Set{Tuple{T,T}}, fold::FoldX{T})::Set{Tuple{T,T}} where T
    new_points = Set{Tuple{T,T}}()
    for point in points
       y = point[2]
       x = point[1]
       if x > fold.pos
          x = 2*fold.pos-x
       end
       push!(new_points, (x,y))
    end
    new_points
end

function FoldPoints(points::Set{Tuple{T,T}}, fold::FoldY{T})::Set{Tuple{T,T}} where T
    new_points = Set{Tuple{T,T}}()
    for point in points
       y = point[2]
       x = point[1]
       if y > fold.pos
          y = 2*fold.pos-y
       end
       push!(new_points, (x,y))
    end
    new_points
end

function SolveProblem(points::Set{Tuple{T,T}}, folds::Vector{Fold{T}}) where T
    println("Day 13 problem 1: ", length(FoldPoints(points, folds[1])))

    # Do all folds
    for fold in folds
        points = FoldPoints(points, fold)
    end

    # Find maxes
    xs = collect(map(t -> t[1], collect(points)))
    max_x = maximum(xs)

    ys = collect(map(t -> t[2], collect(points)))
    max_y = maximum(ys)

    # Render images
    println("Day 13 problem 2")
    for y in 0:max_y
        line = ""
        for x in 0:max_x
            if (x,y) in points
                line = line*"#"
            else
                line = line*" "
            end
        end
        println(line)
    end
end

# Get input filepath
input_filepath = parsed_args["input"]
verbose = parsed_args["verbose"]

points, folds = ProcessFile{Int}(input_filepath)

SolveProblem(points, folds)
