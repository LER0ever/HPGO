((nil . ((eval . (setq
                  projectile-project-test-cmd #'helm-ctest
                  projectile-project-compilation-cmd #'helm-make-projectile
                  projectile-project-compilation-dir "build"
                  helm-make-build-dir (projectile-compilation-dir)
                  helm-ctest-dir (projectile-compilation-dir)
                  ))
         (projectile-project-name . "HPGO")
         (projectile-project-run-cmd . "./HPGO-test --gtest_filter=Algo* --gtest_color=yes")
         (projectile-project-configure-cmd . "cmake -DCMAKE_BUILD_TYPE=Debug -DCMAKE_EXPORT_COMPILE_COMMANDS=ON ..")
         (helm-make-arguments . "-j7"))))
