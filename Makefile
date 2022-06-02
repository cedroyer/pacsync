DESTDIR:=/usr
NAME:=pacsync

install:
	mkdir -p "$(DESTDIR)/bin"
	cat pacsync/{header,libpacsync,main}.bash > "$(DESTDIR)/bin/$(NAME)"
	chmod 755 "$(DESTDIR)/bin/$(NAME)"
