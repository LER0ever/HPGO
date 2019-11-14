(TeX-add-style-hook
 "HPGO"
 (lambda ()
   (TeX-add-to-alist 'LaTeX-provided-class-options
                     '(("article" "11pt")))
   (TeX-add-to-alist 'LaTeX-provided-package-options
                     '(("inputenc" "utf8") ("fontenc" "T1") ("ulem" "normalem")))
   (add-to-list 'LaTeX-verbatim-environments-local "lstlisting")
   (add-to-list 'LaTeX-verbatim-macros-with-braces-local "lstinline")
   (add-to-list 'LaTeX-verbatim-macros-with-braces-local "path")
   (add-to-list 'LaTeX-verbatim-macros-with-braces-local "url")
   (add-to-list 'LaTeX-verbatim-macros-with-braces-local "nolinkurl")
   (add-to-list 'LaTeX-verbatim-macros-with-braces-local "hyperbaseurl")
   (add-to-list 'LaTeX-verbatim-macros-with-braces-local "hyperimage")
   (add-to-list 'LaTeX-verbatim-macros-with-braces-local "hyperref")
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
    "hyperref")
   (LaTeX-add-labels
    "sec:org6926016"
    "sec:org769e836"
    "sec:org338d29f"
    "sec:orgadc01c3"
    "sec:orge4861be"
    "sec:orgb2e1f9f"
    "sec:org44762d0"
    "sec:orga9f34e1"
    "sec:org01adfc1"
    "sec:org06674ed"
    "fig:HPGO-Pipeline-no-network"
    "sec:org6220e3c"
    "fig:HPGO-Pipeline"
    "sec:org8aa1614"
    "sec:org3cae795"
    "sec:orgb81ed4f"
    "sec:org507c172"
    "sec:org30a40d8"
    "sec:org21a40c0"
    "sec:org97bc479"
    "sec:orgfee4535"
    "sec:orgecc4373"))
 :latex)

