(ns quasicrystals.core
  (:import java.awt.image.BufferedImage
           java.io.File 
           javax.imageio.ImageIO
           java.awt.Color)
  (:require [fastmath.core :refer [sin cos asin abs PI constrain floor]])
  (:gen-class))

(def msin (memoize #(sin %)))
(def mcos (memoize #(cos %)))
(def masin (memoize #(asin %)))

(defn wave
  "Returns waveform, the cos of all the y-values rotated by theta and moved
foreward by phase"
  [theta x y phase]
  (let [cth (msin theta)
        sth (mcos theta)]
    (/ (+ (mcos (+ (* cth x) (* sth y) phase)) 1) 2)))

(defn angles
  "Returns a list of n angles between 0 and PI"
  [n]
  (for [m (range n)] (* m (/ PI n))))

(def mangles (memoize angles))

(defn combine
  "Combines a list of values, and wraps the result between 1 and 0"
  [wavs]
  (let [ws (reduce + wavs)]
    (if (odd? (int ws))
      (- 1 (mod ws 1))
      (mod ws 1))))

(defn crystal
  "Returns a vector in the form [x y shade] where shade is the value
(color) at x and y"
  [max-x max-y phase order]
  (for [x (range max-x) y (range max-y)]
    [x y
     (combine
      (map (fn [th] (wave th x y phase))
           (mangles order)))]))

(defn init-image
  "Returns a tuple of a java.awt.image.BufferedImage,
and its java.awt.Graphics2D"
  [w h]
  (let [bi (BufferedImage. w h BufferedImage/TYPE_INT_RGB)
        gfx (.createGraphics bi)]
    [bi gfx]))

(def mshade
  (letfn [(color [s] (int (floor (* 255 (constrain s 0 1)))))]
    (memoize color)))

(defn draw-crystals
  "Draws the crystal qc to the graphics gfx. Scale is the zoom level
r, g, and b are the color offsets, so a r-value of 10 means that the
r component of the color is the shade + 10, modulus 255"
  [wave gfx scale r g b]
  (doseq [[x y shade] wave]
    (let [color (mshade shade)]
      (.setColor gfx (Color. (int (abs (- color r)))
                             (int (abs (- color g)))
                             (int (abs (- color b)))))
      (.fillRect gfx (* x scale) (* y scale) 4 4))))

(defn write-image
  "Writes a image on the imagebuffer bi to the path name"
  [bi name]
  (ImageIO/write bi "png"  (File. (str name "-crystal.png"))))

(defn periodic
  "Swatooth wave going from 0 to 255 and back over m frames, with offset o"
  [n m o]
  (let [pix (* (+ o (/ n m)) PI)]
    (int (abs (* 51 PI (masin (sin pix)))))))

(def mper (memoize periodic))

(defn write-images
  "Writes f frames of animation, that can be looped, to dir. This is the
main function in this program, all of the parameters you should need
can be passed to this function. Defaults: r,g,b offsets: 0; w,h: 200,200
frames of animation: 25; path: current directory"
  [& {:keys [scale order width height frames path]
      :or {scale 3 order 6 width 64 height 36
           frames 10 path ""}}]
  (let [[bi gfx] (init-image width height)]
    (doseq [[p c]
            (for [m (range frames)]
              [(float (* (/ (* 2 PI) frames) m)) m])]
      (draw-crystals
       (crystal (* width scale)
                (* height scale) p order) gfx scale
       (mper c frames 0)
       (mper c frames 0.25)
       (mper c frames 0.5))
      (write-image bi (str path (format "%03d" c)))
      (println (str "Wrote image " c)))))
