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

function ProcessFile{T}(filepath::String)::Vector{T} where T
    open(filepath) do f
        for line in readlines(f)
            return collect(map(s -> parse(T, s), split(line, ',')))
        end
    end
end

# Get input filepath
input_filepath = parsed_args["input"]
verbose = parsed_args["verbose"]

numbers = ProcessFile{UInt}(input_filepath)

pop = Dict{UInt,UInt}()

# Populate dictionary
for num in numbers
    pop[num] = get(pop, num, 0)+1
end

struct count_pop{T} end
function count_pop{T}(pop::Dict{T,UInt}) where T
    sum(values(pop))
end

struct do_step{T} end
function do_step{T}(pop::Dict{T,UInt}) where T
    num_reproducers = get(pop, 0, 0);
    max_lifetime = maximum(collect(keys(pop)))
    cur_lifetimes = sort(collect(filter(n -> n != 0, keys(pop))))

    # Move lifetimes down a notch.
    for l in cur_lifetimes
        pop[l-1] = get(pop, l, 0)
    end
    pop[max_lifetime] = 0;

    # Reproducers
    # The adults
    pop[6] = get(pop, 6, 0)+num_reproducers;
    # The children
    pop[8] = get(pop, 8, 0)+num_reproducers;
end

pop_amounts = Vector{UInt}()
push!(pop_amounts, count_pop{UInt}(pop))

for i in 1:256
    do_step{UInt}(pop)
    push!(pop_amounts, count_pop{UInt}(pop))
end

println("Day 6 problem 1: ", pop_amounts[81])
println("Day 6 problem 2: ", pop_amounts[257])
