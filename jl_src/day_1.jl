
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

let numbers = GetIntArrayFromFile(input_filepath)
    is_first::Bool = true
    num_increases::UInt32 = 0
    last_num::UInt32 = 0

    for num = numbers
        if !is_first
            if num > last_num
                num_increases += 1
            end
        end
        last_num = num
        is_first = false
    end

    println("Day 1: ", num_increases, " increases")
end

