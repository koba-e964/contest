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
using namespace std;
#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

struct pos{
	int r;
	int p;
	int id;
	pos(int r,int p, int id): r(r), p(p), id(id) {}
};

int n,k;

int a[55][1005];

int main(void){
	cin >> n >> k;
	REP(i,0,n) {
		REP(j,0,k) {
			cin >> a[i][j];
		}
	}
	int sum = 0;
	vector<pos> ps;
	set<int> acc;
	set<int> cur;
	REP(i,0,n) { // see [0..i]
		cur.clear();
		int cnt = 0;
		for(int j=0;j<k && cnt < k; j++) {
			REP(t,0,i+1) {
				if (cnt >= k) break;
				int q = a[t][j];
				if(cur.count(q) == 0) {
					cur.insert(q);
					cnt ++;
				}
			}
			cnt += n - i - 1;
		}
		bool first = true;
		for(set<int>::iterator it = cur.begin(); it != cur.end(); ++it) {
			int v = *it;
			if(acc.count(v) == 0) {
				cout << (first ? "" : " ") << v;
				acc.insert(v);
				first = false;
			}
		}
		cout << endl;
	}
}
