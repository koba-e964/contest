#include <algorithm>
#include <cassert>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <iostream>
#include <map>
#include <string>
#include <cstring>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
char s[4000000];
int len;


int check(){
	int cnt=0;
	REP(i,0,len/2){
		if(s[i]!=s[len-i-1])cnt++;
	}
	return cnt;
}


int main(void){
	scanf("%s",s);
	len=strlen(s);
	int res=check();
	if(len%2){//odd
		if(res==0){
			cout<<25*(len-1)<<endl;
			return 0;
		}
		if(res==1){
			cout<<25*len-2<<endl;
			return 0;
		}
		cout<<25*len<<endl;
		return 0;
	}else{
		if(res==0){
			cout<<25*len<<endl;
			return 0;
		}
		if(res==1){
			cout<<25*len-2<<endl;
			return 0;
		}
		cout<<25*len<<endl;
		return 0;
	}
}
