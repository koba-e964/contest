#include <algorithm>
#include <cassert>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <iostream>
#include <map>
#include <string>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;



int main(void){
	string line,rest;
	cin>>line;
	cin>>rest;
	string left,right;
	int ptr=0;
	while(line[ptr]!='|'){
		left+=line[ptr];
		ptr++;
	}
	ptr++;
	while(line[ptr]>0x20){
		right+=line[ptr];
		ptr++;
	}
	int diff=left.length()-right.length();
	int rl=rest.length();
	if(abs(diff)>rl || (diff-rl)%2!=0){
		cout<<"Impossible";
		return 0;
	}
	string o1=left+rest.substr(0,(rl-diff)/2);
	string o2=right+rest.substr((rl-diff)/2,(rl+diff)/2);
	cout<<o1<<"|"<<o2<<endl;
	return 0;
}
