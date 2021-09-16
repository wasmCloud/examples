# interface.mak
#
# common rules for building smithy models
# Some of these may depend on GNUMakefile >= 4.0
#

html_target     ?= html
project_dir     ?= $(abspath $(shell pwd))
codegen_config  ?= $(project_dir)/codegen.toml
top_targets     ?= all build clean lint validate test
WASH            ?= wash

platform_id = $$( uname -s )
platform = $$( \
	case $(platform_id) in \
		( Linux | Darwin | FreeBSD ) echo $(platform_id) ;; \
		( * ) echo Unrecognized Platform;; \
	esac )


# traverse subdirs
.ONESHELL:
ifneq ($(subdirs),)
$(top_targets)::
	for dir in $(subdirs); do \
		$(MAKE) -C $$dir $@; \
	done
endif

all::


clean::
	rm -rf $(html_target)/*.html

ifneq ($(wildcard $(codegen_config)),)
# Run smithy model lint or validation checks
lint validate:: 
	$(WASH) $@ --config $(codegen_config)
endif

ifeq ($(wildcard rust),rust)
# some rules for building rust subdirs
all::
	cd rust && cargo build
test clean clippy::
	cd rust && cargo $@
endif


# for debugging - show variables make is using
make-vars:
	@echo "WASH:          : $(WASH)"
	@echo "codegen_config : $(codegen_config)"
	@echo "platform_id    : $(platform_id)"
	@echo "platform       : $(platform)"
	@echo "project_dir    : $(project_dir)"
	@echo "subdirs        : $(subdirs)"
	@echo "top_targets    : $(top_targets)"


.PHONY: all build release clean lint validate test
