DESTDIR:=/usr
NAME:=pacsync

install:
	mkdir -p "$(DESTDIR)/lib/$(NAME)"
	install -m 755 "pacsync/pacsync" "$(DESTDIR)/lib/$(NAME)"
	install -m 644 "pacsync/libpacsync.bash" "$(DESTDIR)/lib/$(NAME)"
	mkdir -p "$(DESTDIR)/bin"
	sed "s:{{install_dir}}:$(DESTDIR)/lib/$(NAME):g" "pacsync/launcher.bash.tpl" > "$(DESTDIR)/bin/$(NAME)"
	chmod 755 "$(DESTDIR)/bin/$(NAME)"
