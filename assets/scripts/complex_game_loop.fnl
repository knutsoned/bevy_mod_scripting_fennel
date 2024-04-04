(fn on-pre-physics [id]
    (print "on_pre_physics, Handling:")
    (print (string.format "\t-> id: %d" id)))
(fn on-post-physics [id]
    (print "on_post_physics, Handling:")
    (print (string.format "\t-> id: %d" id)))
(fn on-post-update [id]
    (print "on_post_update, Handling:")
    (print (string.format "\t-> id: %d" id)))

{: on-pre-physics : on-post-physics : on-post-update}