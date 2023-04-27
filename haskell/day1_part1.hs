

getFloor :: Int -> [Char] -> Int
getFloor currentFloor [] = currentFloor
getFloor currentFloor instructions = getFloor nextFloor (tail instructions)
    where nextFloor =   if head instructions == '('
                        then currentFloor + 1
                        else currentFloor - 1


run :: IO ()
run = do
    instructions <- readFile "day1.txt"
    let result = getFloor 0 instructions
    print result

tests :: IO()
tests = if all ((== True) . fst) tests
        then putStr "All tests passed ok\n"
        else putStr ("Test failures: \n" ++ failedTestText)
        where
            tests = map (uncurry runTest) [
                ("(())", 0),
                ("()()", 0),
                ("(((", 3),
                ("(()(()(", 3),
                ("))(((((", 3),
                ("())", -1),
                ("))(", -1),
                (")))", -3),
                (")())())", -3)
                ]
            failedTests = filter (\(outcome, _ ) -> not outcome) tests
            failedTestText = concatMap snd failedTests


runTest :: String -> Int -> (Bool, String)
runTest input expected = (actual == expected, "Input: " ++ input ++ " Expected: " ++ show expected ++ " Actual: " ++ show actual ++ "\n")
    where actual = getFloor 0 input