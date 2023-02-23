/*
 * Input: s = "Let's take LeetCode contest"
 * Output: "s'teL ekat edoCteeL tsetnoc"
 * */

#include <assert.h>


char* reverse_string(char*);

int main() {
	char* in = "Let's take LeetCode contest";
	char* out = "s'teL ekat edoCteeL tsetnoc";
	reverse_string(&in);
	assert(in == out);
}

char* reverse_string(char*) {
	
}
