(defn parse-line [line]
  (->> (clojure.string/split line #"\|")
       (map #(clojure.string/trim %1))
       (map #(clojure.string/split %1 #" "))))

(defn parse-input [file]
  (->> (slurp file)
       (clojure.string/split-lines)
       (map parse-line)))

(defn number-of-1478 [digits]
  (->> digits
       (map #(if (contains? #{2 3 4 7} (count %1)) 1 0))))

(defn main [input]
  (->> (map last input)
       (map number-of-1478)
       (flatten)
       (apply +)))

(print "Answer:" (main (parse-input "input.txt")))
