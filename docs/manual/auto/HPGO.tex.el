(TeX-add-style-hook
 "HPGO.tex"
 (lambda ()
   (TeX-add-to-alist 'LaTeX-provided-class-options
                     '(("article" "11pt" "12pt" "letterpaper")))
   (TeX-add-to-alist 'LaTeX-provided-package-options
                     '(("inputenc" "utf8") ("fontenc" "T1") ("ulem" "normalem") ("geometry" "top=2cm" "bottom=4.5cm" "left=2.5cm" "right=2.5cm") ("hyperref" "bookmarksopen=true") ("datetime" "yyyymmdd") ("bookmark" "open")))
   (add-to-list 'LaTeX-verbatim-environments-local "lstlisting")
   (add-to-list 'LaTeX-verbatim-macros-with-braces-local "lstinline")
   (add-to-list 'LaTeX-verbatim-macros-with-braces-local "hyperref")
   (add-to-list 'LaTeX-verbatim-macros-with-braces-local "hyperimage")
   (add-to-list 'LaTeX-verbatim-macros-with-braces-local "hyperbaseurl")
   (add-to-list 'LaTeX-verbatim-macros-with-braces-local "nolinkurl")
   (add-to-list 'LaTeX-verbatim-macros-with-braces-local "url")
   (add-to-list 'LaTeX-verbatim-macros-with-braces-local "path")
   (add-to-list 'LaTeX-verbatim-macros-with-delims-local "lstinline")
   (add-to-list 'LaTeX-verbatim-macros-with-delims-local "path")
   (TeX-run-style-hooks
    "latex2e"
    "article"
    "art11"
    "inputenc"
    "fontenc"
    "graphicx"
    "grffile"
    "longtable"
    "wrapfig"
    "rotating"
    "ulem"
    "amsmath"
    "textcomp"
    "amssymb"
    "capt-of"
    "hyperref"
    "art12"
    "fullpage"
    "geometry"
    "amsthm"
    "amsfonts"
    "amscd"
    "lastpage"
    "enumerate"
    "fancyhdr"
    "mathrsfs"
    "xcolor"
    "svg"
    "listings"
    "datetime"
    "bookmark")
   (TeX-add-symbols
    "doctitle"
    "ID"
    "lstlistingautorefname")
   (LaTeX-add-listings-lstdefinestyles
    "Python"))
 :latex)

