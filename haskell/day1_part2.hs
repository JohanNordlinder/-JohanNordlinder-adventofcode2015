import Test.HUnit

getBasementPosition :: Int -> [Char]-> Int -> Int
getBasementPosition currentFloor instructions instructionCount =
    if currentFloor == -1
    then instructionCount
    else getBasementPosition nextFloor (tail instructions) (instructionCount + 1)
    where nextFloor =   if head instructions == '('
                        then currentFloor + 1
                        else currentFloor - 1

run :: IO ()
run = do
    instructions <- readFile "day1.txt"
    let result = getBasementPosition 0 instructions 0
    print result