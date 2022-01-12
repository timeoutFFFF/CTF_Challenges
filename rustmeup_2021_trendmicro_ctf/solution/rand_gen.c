int main(int argc, char* argv[]){

	int random = atoi(argv[1]);
        
	
	srand(random);
	int ans = rand();
	printf("%d", ans);
	return 0;
}
