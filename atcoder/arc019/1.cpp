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
	string s;
	cin>>s;
	REP(i,0,s.length()){
		switch(s[i]){
		case 'O':s[i]='0';break;
		case 'D':s[i]='0';break;
		case 'I':s[i]='1';break;
		case 'Z':s[i]='2';break;
		case 'S':s[i]='5';break;
		case 'B':s[i]='8';break;
		default:
			break;
		}
	}
	cout<<s<<endl;
}
