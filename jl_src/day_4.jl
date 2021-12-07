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

function ProcessFile{T}(filepath::String) where T
    open(filepath) do f
        lines = readlines(f)
        numbers = map(s -> parse(T, s), split(lines[1], ',')) 
        boards = Vector{Vector{Vector{T}}}()
        i = 3
        while i < length(lines)
            board = Vector{Vector{T}}()
            for j = 0:4
                push!(board, map(s -> parse(T, s), split(lines[i+j])))
            end
            i += 6
            push!(boards, board)
        end
        return (numbers, boards)
    end
end

function BuildBoardPaths(board::Array{T,2}) where T
    paths = Vector{Array{T,1}}()

    for i in 1:5
        push!(paths, board[i,1:5])
        push!(paths, board[1:5,i])
    end
    paths
end

function SolveProblem(numbers::Vector{T}, boards::Vector{Array{T,2}}, board_paths::Vector{Vector{Vector{T}}}) where T
    winner_idxs = Vector{Int}()
    winner_numbers = Vector{T}()
    winner_rem_num = Vector{T}()

    for i in 1:length(numbers)
        nums = numbers[1:i]
        isin_nums = function (n)
            n in nums
        end
        board_func = function (b_ps)
            any(map(p -> all(map(isin_nums, p)), b_ps))
        end
        board_results = map(board_func, board_paths)
        winning_board_idxs = collect(map(t -> t[1], filter(t -> t[2], collect(enumerate(board_results)))))
        winning_board_idxs = collect(filter(i -> !(i in winner_idxs), winning_board_idxs))
        if length(winning_board_idxs) > 0
            last_number = nums[length(nums)]
            for win_idx in winning_board_idxs
                push!(winner_idxs, win_idx)
                push!(winner_numbers, last_number)
                board = boards[win_idx]
                board_rem_numbers = collect(filter(n -> !(n in nums), vec(board)))
                board_rem_sum = sum(board_rem_numbers)
                push!(winner_rem_num, board_rem_sum)
            end
        end

    end

    println("Day 4 problem 1: ", winner_numbers[1]*winner_rem_num[1])
    len = length(winner_numbers)
    println("Day 4 problem 2: ", winner_numbers[len]*winner_rem_num[len])
end

# Get input filepath
input_filepath = parsed_args["input"]
verbose = parsed_args["verbose"]

# Process input data
(numbers, boards) = ProcessFile{Int32}(input_filepath)

# Transform data into arrays
boards = map(b -> cat(b...; dims=2), boards)

# build paths for all boards
board_paths = map(b -> BuildBoardPaths(b), boards)

SolveProblem(numbers, boards, board_paths)
