#include <algorithm>
#include <cassert>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <iostream>
#include <map>
#include <set>
#include <string>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
const double EPS=1e-9;



int main(void){
	string s;
	set<string> mm;
	while(cin>>s) {
		int i= 0;
		int t = -1;
		while(i < s.length()) {
			if(s[i] == '@') {
				if(t != -1) {
					mm.insert(s.substr(t+1,i-t-1));
				}
				t = i;
			}
			i ++;
		}
		if(t != -1) {
			mm.insert(s.substr(t+1,i-t-1));
		}
	}
	for(set<string>::iterator it = mm.begin(); it != mm.end(); it++) {
		if( *it != "")
			cout << *it << endl;
	}
}
