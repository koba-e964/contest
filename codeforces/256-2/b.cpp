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
const int N = 120;

string orig, to;
int dp[N];

int ocnt[26];
int tcnt[26];

int main(void){
	cin>>orig>>to;
	REP(i,0,orig.size()) {
		ocnt[orig[i] - 'a'] ++;
	}
	REP(i,0,to.size()) {
		tcnt[to[i] - 'a'] ++;
	}
	// array?
	if (orig.size() == to.size()) {
		bool ok = true;
		REP(i,0,26) {
			ok &= ocnt[i] == tcnt[i];
		}
		if(ok) {
			cout << "array" <<endl;
			return 0;
		}
	}
	//suffix?
	int pos = 0;
	for(int i=0; i< orig.size() && pos < to.size(); i++) {
		if (orig[i] == to[pos]) {
			pos ++;
		}
	}
	if (pos == to.size()) {
		cout << "automaton" <<endl;
		return 0;
	}
	//both?
	bool ok = true;
	REP(i,0,26) {
		ok &= ocnt[i] >= tcnt[i];
	}
	if(ok) {
		cout << "both" <<endl;
		return 0;
	}
	cout << "need tree" << endl;
	return 0;
}
