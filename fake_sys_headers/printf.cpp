#include <stdarg.h>

extern "C" void log_msg(const char*);
extern "C" void log_fatal(const char*);

extern "C" int pf_write_float(char*,float);
extern "C" int pf_write_int(char*,int);

char PRINTF_BUFFER[1024];

int vprintf ( const char * format, va_list arg ) {
	int n=0;
	for (int i=0;format[i];n++,i++) {
		if (format[i] == '%') {
			i++;
			for (;;i++) {
				char c = format[i];
				if (c == '%') {
					PRINTF_BUFFER[n] = c;
				} else if (c == '.' || (c >= '0' && c <= '9') || c=='l') {
					// ...
				} else if (c == 'e' || c == 'f') {
					n += pf_write_float(PRINTF_BUFFER+n,va_arg(arg,double))-1;
					break;
				} else if (c == 'd' || c == 'i') {
					n += pf_write_int(PRINTF_BUFFER+n,va_arg(arg,int))-1;
					break;
				} else if (c == 's') {
					char* string = va_arg(arg,char*);
					int j=0;
					while (string[j] != 0) {
						PRINTF_BUFFER[n++] = string[j++];
					}
					n--;
					break;
				} else {
					PRINTF_BUFFER[n++] = c;
					PRINTF_BUFFER[n++] = '<';
					PRINTF_BUFFER[n++] = '<';
					PRINTF_BUFFER[n++] = '<';
					PRINTF_BUFFER[n] = 0;
					log_fatal(PRINTF_BUFFER);
				}
			}
		} else {
			PRINTF_BUFFER[n] = format[i];			
		}
	}
	PRINTF_BUFFER[n] = 0;
	log_msg(PRINTF_BUFFER);
	//for (unsigned volatile int i=0;i<10000000;i++);
	return n;
}