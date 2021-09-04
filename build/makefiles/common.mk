top_targets     ?= all clean test lint validate

# traverse subdirs
.ONESHELL:
ifneq ($(subdirs),)
$(top_targets)::
	for dir in $(subdirs); do \
		$(MAKE) -C $$dir $@ ; \
	done
endif

.PHONY: all clean test lint validate
