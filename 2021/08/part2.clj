(require 'clojure.set)
(require 'clojure.string)

(defn parse-line [line]
  (->> (clojure.string/split line #"\|")
       (map #(clojure.string/trim %1))
       (map #(map set (clojure.string/split %1 #" ")))))

(defn parse-input [file]
  (->> (slurp file)
       (clojure.string/split-lines)
       (map parse-line)))


(defn first-subset [set2 coll] (first (filter #(clojure.set/subset? %1 set2) coll)))
(defn first-superset [set2 coll] (first (filter #(clojure.set/superset? %1 set2) coll)))
(defn first-not-superset [set2 coll] (first (filter #(not (clojure.set/superset? %1 set2)) coll)))


(defn build-table [digits]
  (let [by-count (group-by count digits)]
    (let [one (first (get by-count 2))
          four (first (get by-count 4))
          seven (first (get by-count 3))
          eight (first (get by-count 7))]
      (let [nine (first-superset four (get by-count 6))
            six (first-not-superset one (get by-count 6))
            three (first-superset one (get by-count 5))]
        (let [five (first-subset six (get by-count 5))]
          (let [two (first (filter
                             #(and
                                (not= %1 three)
                                (not= %1 five))
                             (get by-count 5)))
                zero (first (filter
                             #(and
                                (not= %1 six)
                                (not= %1 nine))
                             (get by-count 6)))
                ]
            {zero 0, one 1, two 2, three 3, four 4
             five 5, six 6, seven 7, eight 8, nine 9}))))))


(defn main [input]
  (->> input
       (map (fn [line]
              (let [table (build-table (first line))]
                (let [digits (map #(get table %1) (second line))]
                  (+ (* 1000 (first digits))
                     (* 100 (nth digits 1))
                     (* 10 (nth digits 2))
                     (* 1 (nth digits 3))
                     )))))
            (apply +)))

(print "Answer:" (main (parse-input "input.txt")))
