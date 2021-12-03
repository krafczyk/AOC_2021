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
function ProcessFile(filepath::String)::Vector{String}
    numbers = Vector{String}()
    open(filepath) do f
        for line in readlines(f)
            push!(numbers, line)
        end
    end
    return numbers
end

struct problem_1{T} end

function problem_1{T}(numbers::Vector{String}) where T <: Unsigned
    str_len = length(numbers[1])
    zero_count = zeros(T, str_len)
    one_count = zeros(T, str_len)
    for num_string in numbers
        for i in 1:str_len
            if num_string[i] == '0'
                zero_count[i] += 1
            elseif num_string[i] == '1'
                one_count[i] += 1
            else
                println("Unknown chracter!")
            end
        end
    end
    gamma_rate = ""
    epsilon_rate = ""
    for i in 1:str_len
        if zero_count[i] > one_count[i]
            gamma_rate = gamma_rate * '0'
            epsilon_rate = epsilon_rate * '1'
        else
            gamma_rate = gamma_rate * '1'
            epsilon_rate = epsilon_rate * '0'
        end
    end
    gamma_rate = parse(T, gamma_rate, base=2)
    epsilon_rate = parse(T, epsilon_rate, base=2)
    power_rate = gamma_rate*epsilon_rate
    println("Day 3 problem 1: ", power_rate)
end

struct find_o2gen_rating{T} end

function find_o2gen_rating{T}(numbers::Vector{String}, idx::UInt)::T where T <: Unsigned
    if length(numbers) == 1
        return parse(T, numbers[1], base=2)
    else
        num_zeros::UInt = 0
        num_ones::UInt = 0
        for num_string in numbers
            if num_string[idx] == '0'
                num_zeros += 1
            elseif num_string[idx] == '1'
                num_ones += 1
            else
                println("Unknown character!")
            end
        end
        new_list = Vector{String}()
        if num_zeros > num_ones
            for num_string in numbers
                if num_string[idx] == '0'
                    push!(new_list, num_string)
                end
            end
        else
            for num_string in numbers
                if num_string[idx] == '1'
                    push!(new_list, num_string)
                end
            end
        end
        return find_o2gen_rating{T}(new_list, idx+1)
    end
end

struct find_co2_rating{T} end

function find_co2_rating{T}(numbers::Vector{String}, idx::UInt)::T where T <: Unsigned
    if length(numbers) == 1
        return parse(T, numbers[1], base=2)
    else
        num_zeros::UInt = 0
        num_ones::UInt = 0
        for num_string in numbers
            if num_string[idx] == '0'
                num_zeros += 1
            elseif num_string[idx] == '1'
                num_ones += 1
            else
                println("Unknown character!")
            end
        end
        new_list = Vector{String}()
        if num_zeros > num_ones
            for num_string in numbers
                if num_string[idx] == '1'
                    push!(new_list, num_string)
                end
            end
        else
            for num_string in numbers
                if num_string[idx] == '0'
                    push!(new_list, num_string)
                end
            end
        end
        return find_co2_rating{T}(new_list, idx+1)
    end
end

struct problem_2{T} end

function problem_2{T}(numbers::Vector{String}) where T <: Unsigned
    o2gen_rating = find_o2gen_rating{T}(numbers, convert(UInt, 1))
    co2_rating = find_co2_rating{T}(numbers, convert(UInt, 1))
    lifesupport_rating = o2gen_rating*co2_rating
    println("Day 3 problem 2: ", lifesupport_rating)
end

# Get input filepath
input_filepath = parsed_args["input"]
verbose = parsed_args["verbose"]

numbers = ProcessFile(input_filepath)

problem_1{UInt32}(numbers)
problem_2{UInt32}(numbers)
