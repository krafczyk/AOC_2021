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

abstract type Location{T} end

mutable struct LocationA{T} <: Location{T}
    x::T
    depth::T
    LocationA{T}(x,y) where {T <: Number} = new(x,y)
end

mutable struct LocationB{T} <: Location{T}
    x::T
    depth::T
    aim::T
    LocationB{T}(x,y,z) where {T <: Number} = new(x,y,z)
end

abstract type Instruction{T} end

struct Down{T} <: Instruction{T}
    val::T
end

function execute(loc::LocationA{T}, inst::Down{T})::LocationA{T} where T <: Number
    return LocationA{T}(loc.x, loc.depth+inst.val)
end

function execute(loc::LocationB{T}, inst::Down{T})::LocationB{T} where T <: Number
    return LocationB{T}(loc.x, loc.depth, loc.aim+inst.val)
end

struct Up{T} <: Instruction{T}
    val::T
end

function execute(loc::LocationA{T}, inst::Up{T})::LocationA{T} where T <: Number
    return LocationA{T}(loc.x, loc.depth-inst.val)
end

function execute(loc::LocationB{T}, inst::Up{T})::LocationB{T} where T <: Number
    return LocationB{T}(loc.x, loc.depth, loc.aim-inst.val)
end

struct Forward{T} <: Instruction{T}
    val::T
end

function execute(loc::LocationA{T}, inst::Forward{T})::LocationA{T} where T <: Number
    return LocationA{T}(loc.x+inst.val, loc.depth)
end

function execute(loc::LocationB{T}, inst::Forward{T})::LocationB{T} where T <: Number
    return LocationB{T}(loc.x+inst.val, loc.depth+inst.val*loc.aim, loc.aim)
end

# Parse arguments
parsed_args = ArgParse.parse_args(ARGS, s)

struct ProcessFile{T} end

# Process files
function ProcessFile{T}(filepath::String)::Vector{Instruction{T}} where T<: Number
    result = Vector{Instruction{T}}()
    open(filepath) do f
        for line in readlines(f)
            m = match(r"(forward|up|down) ([0-9]*)", line)
            if m !== nothing
                inst = m.captures[1]
                val = parse(T, m.captures[2])
                if inst == "forward"
                    push!(result, Forward{T}(val))
                elseif inst == "up"
                    push!(result, Up{T}(val))
                elseif inst == "down"
                    push!(result, Down{T}(val))
                end
            end
        end
    end
    return result
end

# Get input filepath
input_filepath = parsed_args["input"]
verbose = parsed_args["verbose"]

# Process files
instructions = ProcessFile{Int32}(input_filepath)

function SolveDay1(instructions::Vector{Instruction{T}}) where T<: Number
    location = LocationA{T}(0,0)
    for inst in instructions
        location = execute(location, inst)
    end
    println("Day 2 problem 1: ", location.x*location.depth)
end

function SolveDay2(instructions::Vector{Instruction{T}}) where T<: Number
    location = LocationB{T}(0,0,0)
    for inst in instructions
        location = execute(location, inst)
    end
    println("Day 2 problem 2: ", location.x*location.depth)
end

SolveDay1(instructions)
SolveDay2(instructions)
