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

const int N=10010;
const int DEBUG = 0;

int n;
int r[N];

vector<int> pile[3];
vector<pair<int,int> > motions;

void mv(int a,int b) {
	motions.push_back(pair<int,int>(a,b));
	int t = pile[a-1].back();
	pile[a-1].pop_back();
	pile[b-1].push_back(t);
}
int top(int a) {
	return pile[a-1].back();
}

void out() {
	cout << motions.size() << endl;
	REP(i,0,motions.size()) {
		pair<int,int> r = motions[i];
		cout << r.first << " "  << r.second << endl;
	}
}

void qs(int st,int en,int from, int to, int d) {
	if(en == st) {
		mv(from,to);
		return;
	}
	if(en < st) {return;}
	int piv = (st + en + 1) / 2;
	int d2 = 6 - from - to;
	REP(c,st,en+1) {
		int t = top(from);
		if(t >= piv) {
			mv(from, d ? to :d2);
		} else {
			mv(from, d ? d2 :to);
		}
	}
	if(d ==0 ) {
		qs(st,piv-1,to,from,1-d);
		qs(piv,en,d2,to,d);
		REP(i,st, piv) mv(from, to);
	} else {
		qs(piv,en,to,from,0);
		qs(st,piv-1,d2,to,1);
		REP(i,piv,en+1) mv(from,to);
	}
}

int main(void){
	cin>>n;
	REP(i,0,n) {
		cin>>r[i];
		pile[0].push_back(r[i]);
	}
	qs(1,n,1,2,0);
	out();
	if(DEBUG) {
		REP(i,0,3) {
			cout << "pile[" << i << "]:";
			REP(j,0,pile[i].size()) {
				cout << pile[i][j] << " ";
			}
			cout << endl;
		}
	}
}
