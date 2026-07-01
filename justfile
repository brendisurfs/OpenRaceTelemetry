build:
  idf.py build

flash: build
  idf.py flash monitor

clean: 
  idf.py fullclean
