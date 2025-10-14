;;; Directory Local Variables for pokefight project
;;; For more information see (info "(emacs) Directory Variables")

((nil . ((eval . (dap-register-debug-template
                  "Pokefight Debug"
                  (list :type "lldb"
                        :request "launch"
                        :name "Pokefight Debug"
                        :program (concat (projectile-project-root) "target/debug/pokefight")
                        :cwd (projectile-project-root)))))))
