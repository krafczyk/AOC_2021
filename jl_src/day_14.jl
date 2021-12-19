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
function ProcessFile(filepath::String)::Tuple{String,Vector{Tuple{String,String}}}
    open(filepath) do f
        lines = readlines(f)
        rules = Vector{Tuple{String,String}}()

        init_formula = lines[1]

        for line in lines[3:end]
            split_line = split(line)
            left_val = split_line[1]
            right_val = split_line[3]
            push!(rules, (left_val, right_val))
        end
        return (init_formula, rules)
    end
end

function BuildRuleTable(in_rules::Vector{Tuple{String,String}})::Dict{String,Tuple{String,String}}
    rules = Dict{String,Tuple{String,String}}()
    for rule in in_rules
        left = rule[1]
        right = rule[2]
        left_pat = left[1]*right[1]
        right_pat = right[1]*left[2]
        rules[left] = (left_pat, right_pat)
    end
    rules
end

struct FormulaToTokens{T} end
function FormulaToTokens{T}(formula::String)::Dict{String,T} where T
    tokens = Dict{String,T}()
    for i in 1:(length(formula)-1)
        token = formula[i]*formula[i+1]
        tokens[token] = get(tokens, token, 0)+1
    end
    tokens
end

function ExpandFormula(formula::Dict{String,T}, rules::Dict{String,Tuple{String,String}})::Dict{String,T} where T
    new_formula = Dict{String,T}()
    for (key, val) in formula
        new_keys = rules[key]
        new_formula[new_keys[1]] = get(new_formula, new_keys[1], 0)+val
        new_formula[new_keys[2]] = get(new_formula, new_keys[2], 0)+val
    end
    new_formula
end

function CharCount(formula_tokens::Dict{String,T}, first_char::Char)::Dict{Char,T} where T
    char_count = Dict{Char,T}()
    char_count[first_char] = get(char_count, first_char, 0)+1
    for (token, num) in formula_tokens
       the_char = token[2]
       char_count[the_char] = get(char_count, the_char, 0)+num
    end
    char_count
end

function ComputeDiff(char_count::Dict{Char,T})::T where T
    min_v = minimum(values(char_count))
    max_v = maximum(values(char_count))
    return max_v-min_v
end

function SolveProblem(formula_tokens::Dict{String,T}, rules::Dict{String,Tuple{String,String}}) where T
    counts = Vector{UInt}()
    push!(counts, ComputeDiff(CharCount(formula_tokens, first_char)))

    num_steps = 40

    for _ in 1:num_steps
        formula_tokens = ExpandFormula(formula_tokens, rules)
        push!(counts, ComputeDiff(CharCount(formula_tokens, first_char)))
    end

    println("Day 14 problem 1: ", counts[11]);
    println("Day 14 problem 2: ", counts[41]);
end

# Get input filepath
input_filepath = parsed_args["input"]
verbose = parsed_args["verbose"]

init_formula, rules = ProcessFile(input_filepath)
first_char = init_formula[1]

rules = BuildRuleTable(rules)
formula_tokens = FormulaToTokens{UInt}(init_formula)

SolveProblem(formula_tokens, rules)
