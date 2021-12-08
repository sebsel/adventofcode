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
  (def by-count (group-by count digits))
  )

  ; (def recipes {
  ;   1 (fn [] (first (get by-count 2)))
  ;   4 (fn [] (first (get by-count 4)))
  ;   7 (fn [] (first (get by-count 3)))
  ;   8 (fn [] (first (get by-count 7)))
  ;   9 (fn [] (first-superset (first (get by-count 4)) (get by-count 6)))
  ;   6 (fn [] (first-not-superset (first (get by-count 2)) (get by-count 6)))
  ;   3 (fn [] (first-superset (first (get by-count 2)) (get by-count 5)))

  ;   ; 5 (fn [table] (first-subset (get table 6) (get by-count 5)))
  ;   ; 2 (fn [table] (first (filter
  ;   ;                        #(and
  ;   ;                           (not= %1 (get table 3))
  ;   ;                           (not= %1 (get table 5)))
  ;   ;                        (get by-count 5))))
  ;   ; 0 (fn [table] (first (filter
  ;   ;                        #(and
  ;   ;                           (not= %1 (get table 6))
  ;   ;                           (not= %1 (get table 6)))
  ;   ;                        (get by-count 6))))
  ;   })
  ; (reduce (fn [table digit] (assoc table ((second digit)) (first digit))) {} recipes))

(defn main [input]
  (->> input (map (fn [line]
                    (def table (build-table (first line)))
                    (println table)
                    (println (second line))
                    (def digits (map #(get table %1) (second line))) digits))
                    ; (+ (* 1000 (first digits))
                    ;    (* 100 (nth digits 1))
                    ;    (* 10 (nth digits 2))
                    ;    (* 1 (nth digits 3))
                    ;    )))
       ; (apply +)
       ))

  ; (->> (map last input)
       ; (map #(detect-number %1 (build-hash (map last input))))
       ; (apply +)
  ; ))

(print "Answer:" (main (parse-input "input.txt")))
