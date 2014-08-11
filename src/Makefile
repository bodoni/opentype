all: bins

bins: libs
	@$(MAKE) -C bins all

libs:
	@$(MAKE) -C libs all

clean:
	@$(MAKE) -C bins clean
	@$(MAKE) -C libs clean

.PHONY: all bins libs clean
