(ns day13.core
  (:require [clojure.java.io :as io])
  (:gen-class))

(defn read-lines []
  (with-open [rdr (io/reader "input.txt")]
    (reduce conj [] (map char-array (line-seq rdr)))))

(defn search-row [row x]
  (if (< x (count row))
    (if (= (nth row x) \S)
      x
      (search-row row (+ x 1)))
    -1))

(defn find-s [pipes y]
  (let [x (search-row (nth pipes y) 0)]
    (if (= x -1)
      (find-s pipes (+ y 1))
      [x y])))

(defn find-cycle [pipes spx spy sx sy]
  (loop [px spx py spy x sx y sy l 1 path #{[spx spy]}]
    (let [p (nth (nth pipes y) x)]
      (if (and (not= p \S) (not= l -1))
        (cond
          (and (= p \|) (and (= px x) (= (- py 1) y))) (recur x y x (- y 1) (+ l 1) (conj path [x y]))
          (and (= p \|) (and (= px x) (= (+ py 1) y))) (recur x y x (+ y 1) (+ l 1) (conj path [x y]))
          (and (= p \-) (and (= (- px 1) x) (= py y))) (recur x y (- x 1) y (+ l 1) (conj path [x y]))
          (and (= p \-) (and (= (+ px 1) x) (= py y))) (recur x y (+ x 1) y (+ l 1) (conj path [x y]))
          (and (= p \L) (and (= px x) (= (+ py 1) y))) (recur x y (+ x 1) y (+ l 1) (conj path [x y]))
          (and (= p \L) (and (= (- px 1) x) (= py y))) (recur x y x (- y 1) (+ l 1) (conj path [x y]))
          (and (= p \J) (and (= px x) (= (+ py 1) y))) (recur x y (- x 1) y (+ l 1) (conj path [x y]))
          (and (= p \J) (and (= (+ px 1) x) (= py y))) (recur x y x (- y 1) (+ l 1) (conj path [x y]))
          (and (= p \7) (and (= (+ px 1) x) (= py y))) (recur x y x (+ y 1) (+ l 1) (conj path [x y]))
          (and (= p \7) (and (= px x) (= (- py 1) y))) (recur x y (- x 1) y (+ l 1) (conj path [x y]))
          (and (= p \F) (and (= (- px 1) x) (= py y))) (recur x y x (+ y 1) (+ l 1) (conj path [x y]))
          (and (= p \F) (and (= px x) (= (- py 1) y))) (recur x y (+ x 1) y (+ l 1) (conj path [x y]))
          :else (recur 0 0 0 0 -1 {}))
        {:l l :p path}))))

(defn coordinates [c]  ; produce all combinations of elements in a collection
  (for [x (range c)  y (range c)] [x y]))

(defn ray-search [x_ y_ dx dy pipes path]
  (let [isPath (some (zipmap [[x_ y_]] (repeat true)) path) h (count pipes) w (count (nth pipes 0))]
    (if (= isPath nil)
      (loop [x x_ y y_ i 0]
        (if (and (>= y 0) (< y h) (>= x 0) (< x w))
          (let [in (some (zipmap [[x y]] (repeat true)) path) p (nth (nth pipes y) x)]
            (cond
              (and (not= in nil) (= p \|) (not= dx 0)) (recur (+ x dx) y (+ i 1))
              (and (not= in nil) (= p \-) (not= dy 0)) (recur x (+ y dy) (+ i 1))
              :else (recur (+ x dx) (+ y dy) i)))
          i))
      0)))

(defn enclosed [pipes path]
  (reduce + (map (fn [[x y]] (if (every? odd? (map (fn [[dx dy]] (let [i (ray-search x y dx dy pipes path)] i)) [[-1 0] [1 0] [0 -1] [0 1]])) 1 0)) (coordinates (count pipes)))))

(defn solution-part1 [pipes snd]
  (let [[x y] (find-s pipes 0) r (apply max-key :l (map (fn [[dx dy]] (find-cycle pipes x y (+ x dx) (+ y dy))) [[-1 0] [1 0] [0 -1] [0 1]]))]
    (if snd
      (enclosed pipes (r :p))
      (/ (r :l) 2))))

(defn -main
  "AoC Day 10"
  [& args]
  (let [part (or (System/getenv "part") "part1")
        input (read-lines)]
    (do
      (println (condp = part
                 "part1" (solution-part1 input false)
                 "part2" (solution-part1 input true)
                 (str "Unknown part " part))))))
