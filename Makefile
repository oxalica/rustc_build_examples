examples = $(shell find . -mindepth 1 -maxdepth 1 -type d -not -name '.*' | sort)
cleans = $(addsuffix .clean,$(examples))

.PHONY: all clean $(examples) $(cleans)

all: $(examples)

$(examples):
	$(MAKE) -C $@

clean: $(cleans)

$(cleans):
	$(MAKE) -C $(basename $@) clean
