
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

# Get input filepath
input_filepath = parsed_args["input"]
verbose = parsed_args["verbose"]

# Process files
function GetIntArrayFromFile(filepath::String)
    result = Array{UInt32, 1}()
    open(input_filepath) do f
        for line in readlines(f)
            num = parse(UInt32, line)
            append!(result, [num])
        end
    end
    return result
end

function WindowIncreases(numbers::Array{T, 1}, window_size::UInt32) where T <: Number
    is_first::Bool = true
    num_increases::UInt32 = 0
    last_sum::T = 0

    for i in 1:length(numbers)-(window_size-1)
        sum_val = sum(numbers[i:i+(window_size-1)])
        if !is_first
            if sum_val > last_sum
                num_increases += 1
            end
        end
        last_sum = sum_val
        is_first = false
    end
    return num_increases
end

let numbers = GetIntArrayFromFile(input_filepath)
    println("Day 1 part 1: ", WindowIncreases(numbers, UInt32(1)), " increases")
    println("Day 1 part 2: ", WindowIncreases(numbers, UInt32(3)), " increases")
end

