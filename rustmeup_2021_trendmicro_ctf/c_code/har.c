
#include <stdio.h>
#include <stddef.h>
#include <string.h>
#include <stdlib.h>


#define KEY "\xc7\xda\x39\x6b"
#define FLAG "readme"

// RC4 encryption
unsigned char* rc4(unsigned char* data ){
	
	unsigned char S[256];
	unsigned char tmp;
	int i, j;

	size_t key_len; 
	size_t data_len;

	key_len  = strlen(KEY);
	data_len = strlen(data);

	unsigned char* out = (unsigned char*)malloc(data_len);


	// initialize S
	for(i = 0; i < 256; i++ ){
		S[i] = i;
	}

	j = 0;
	for(i = 0; i < 256; i++){
		j = ((int)(j  + S[i] + KEY[i % key_len]) % 256);
		
		// Swapping
		tmp  = S[j];
		S[j] = S[i];
		S[i] = tmp;
	}

	j = 0;
	i = 0;

	for(int k = 0; k < data_len; k++) {
		i = (i + 1) % 256;
		j = (int)(j + S[i]) % 256;

		tmp  = S[j];
		S[j] = S[i];
		S[i] = tmp;

		out[k] = (int)data[k] ^ (int)S[((int)S[i] + (int)S[j]) % 256];
	}
	
	return out;
}

// stdout encrypted text
void print_rc4(char* data) {
	size_t data_len = strlen(data);
	char* encrypted = rc4(data);
	printf("%s", data);
	//printf("\n");
}

// reads flag
void file_read(){
	FILE *fp;
	char flag[256] = {0};
	fp = fopen(FLAG, "r");
	fgets(flag, 256, (FILE*)fp);
	printf(flag);
	fflush(stdout);
	fclose(fp);
}


// Random number generator
int get_meaning(){
	srand(time(0));
        int random = rand();
	return random;
}

// example function
void you_get_rce(){
	printf("You got RCE!");
}

int main(){

	printf("What's the key to unlock the flag");
	fflush(stdout);
	
	int your_key;
	int flag_key;

	flag_key = get_meaning();

	scanf("%d", &your_key);

	if(your_key == flag_key){
		file_read();
	}else {
		printf("Incorrect key!!");
		fflush(stdout);
	}

}
