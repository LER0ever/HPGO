(TeX-add-style-hook
 "HPGO"
 (lambda ()
   (TeX-add-to-alist 'LaTeX-provided-class-options
                     '(("article" "12pt" "letterpaper")))
   (TeX-add-to-alist 'LaTeX-provided-package-options
                     '(("geometry" "top=2cm" "bottom=4.5cm" "left=2.5cm" "right=2.5cm") ("hyperref" "bookmarksopen=true") ("datetime" "yyyymmdd") ("bookmark" "open")))
   (add-to-list 'LaTeX-verbatim-environments-local "lstlisting")
   (add-to-list 'LaTeX-verbatim-macros-with-braces-local "hyperref")
   (add-to-list 'LaTeX-verbatim-macros-with-braces-local "hyperimage")
   (add-to-list 'LaTeX-verbatim-macros-with-braces-local "hyperbaseurl")
   (add-to-list 'LaTeX-verbatim-macros-with-braces-local "nolinkurl")
   (add-to-list 'LaTeX-verbatim-macros-with-braces-local "url")
   (add-to-list 'LaTeX-verbatim-macros-with-braces-local "path")
   (add-to-list 'LaTeX-verbatim-macros-with-braces-local "lstinline")
   (add-to-list 'LaTeX-verbatim-macros-with-delims-local "path")
   (add-to-list 'LaTeX-verbatim-macros-with-delims-local "lstinline")
   (TeX-run-style-hooks
    "latex2e"
    "article"
    "art12"
    "fullpage"
    "geometry"
    "amsmath"
    "amsthm"
    "amsfonts"
    "amssymb"
    "amscd"
    "lastpage"
    "enumerate"
    "fancyhdr"
    "mathrsfs"
    "xcolor"
    "graphicx"
    "svg"
    "listings"
    "hyperref"
    "datetime"
    "bookmark")
   (TeX-add-symbols
    "doctitle"
    "ID"
    "lstlistingautorefname")
   (LaTeX-add-listings-lstdefinestyles
    "Python"))
 :latex)

