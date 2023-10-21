NAME		=	fb_cdn

CC			=	clang++

CFLAGS		=	-Wall -Wextra -Werror -std=c++11 -I. -march=haswell
ifeq ($(DEBUG), 1)
	CFLAGS		+= -DDEBUG_LOG=1
endif

LDFLAGS		=	-lpthread

SRCS		=	 main.cpp \
file_manager.cpp

OBJS		=	$(SRCS:.cpp=.o)
DEPS		=	$(OBJS:.o=.d)

%.o : %.cpp
	$(CC) $(CFLAGS) -MMD -MP -c $< -o $@

all			:	$(NAME)

$(NAME)		:	$(OBJS)
				$(CC) $(CFLAGS) $(OBJS) -o $(NAME) $(LDFLAGS)

clean		:
				rm -rf $(OBJS) $(DEPS)

fclean		:	clean
				rm -f $(NAME)

re			:	fclean
				$(MAKE)

debug		:	fclean
				$(MAKE) DEBUG=1

-include $(DEPS)

.PHONY: all clean fclean re -include debug
