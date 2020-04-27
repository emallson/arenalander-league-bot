--
-- PostgreSQL database dump
--

-- Dumped from database version 12.2 (Debian 12.2-2.pgdg100+1)
-- Dumped by pg_dump version 12.2 (Debian 12.2-4)

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- Data for Name: __diesel_schema_migrations; Type: TABLE DATA; Schema: public; Owner: -
--

COPY public.__diesel_schema_migrations (version, run_on) FROM stdin;
00000000000000	2020-04-26 02:31:14.149139
20200422225049	2020-04-26 02:37:44.655145
20200423211127	2020-04-26 02:37:44.690047
20200424205441	2020-04-26 02:37:44.762355
20200425191635	2020-04-26 02:38:07.754894
\.


--
-- Data for Name: leagues; Type: TABLE DATA; Schema: public; Owner: -
--

COPY public.leagues (id, title, start_date, end_date) FROM stdin;
1	April 2020 Test	2020-04-01 00:00:00+00	2020-05-01 00:00:00+00
\.


--
-- Data for Name: users; Type: TABLE DATA; Schema: public; Owner: -
--

COPY public.users (id, discordid, name) FROM stdin;
2	0	alpha
3	1	beta
1	2	gamma
5	3	delta
6	4	epsilon
7	5	zeta
8	6	eta
\.


--
-- Data for Name: decks; Type: TABLE DATA; Schema: public; Owner: -
--

COPY public.decks (id, league, owner, creation_date, resigned, active) FROM stdin;
2	1	2	2020-04-26 16:43:54.360955+00	f	t
1	1	1	2020-04-26 02:41:21.423658+00	t	f
3	1	3	2020-04-26 16:57:11.619528+00	f	t
6	1	6	2020-04-26 19:13:22.978591+00	f	t
5	1	5	2020-04-26 17:49:38.745541+00	t	f
7	1	7	2020-04-26 20:31:19.767751+00	t	f
8	1	8	2020-04-27 18:37:52.410066+00	f	t
4	1	1	2020-04-26 17:17:42.540325+00	f	f
\.


--
-- Data for Name: deck_contents; Type: TABLE DATA; Schema: public; Owner: -
--

COPY public.deck_contents (id, deck, card, count) FROM stdin;
1	1	2ac93092-1a57-4cf7-8f03-1dca86d5c476	1
2	1	bc71ebf6-2056-41f7-be35-b2e5c34afa99	11
3	1	ea27c8bf-33c5-443f-b8b2-51235efc2491	1
4	1	b9243163-b726-432e-830f-86132aa7f34a	1
5	1	48a99dce-0aa9-4aac-81df-cec5f94c639d	1
6	1	a0c47ab6-dfb4-46ee-a3f7-9e1521b4bb4b	1
7	1	16f93781-6740-40c7-a727-7911beac4e74	1
8	1	592c91fc-6430-4c76-9460-65f047350f67	1
9	1	973fd4d4-9255-4825-85b7-503606c4e932	1
10	1	a8fdbcdf-479d-4582-9ad5-9fbd4c740c29	1
11	1	b2c6aa39-2d2a-459c-a555-fb48ba993373	12
12	1	59c7496b-19a2-478b-b28f-6f153c2458ae	1
13	1	00187de2-bc48-4137-97d8-a9a0fafc76c1	1
14	1	2f990b54-fbf3-4949-85bb-9ba39710e72a	1
15	1	1f9efad1-9f51-47aa-a5a7-9c8103435a01	1
16	1	319d5cf8-ab72-401e-b04e-c13a7a6c1aac	1
17	1	b872f147-00eb-4cf8-af8c-4144356b2089	1
18	1	c7b044c3-3cfa-407e-bf20-2875e8e04b7b	1
19	1	1e825298-7c99-40cc-81c9-280cc7ed98d3	1
20	1	fa1b722a-109e-4a43-bd7b-818292728cb3	1
21	1	c623aeb1-e6d4-48fe-bd2a-a7a6729aa4df	1
22	1	7cfc14ce-9940-4c61-9daa-2c6dbf1a80ad	1
23	1	41d11144-32fc-4e45-a8db-3edb9dc0ce80	1
24	1	ab5ebae2-cd77-4a7d-a93b-8042cd486429	1
25	1	0c85b8f7-0bd0-4680-9ec5-d4b110460a54	1
26	1	a75445d3-1303-4bb5-89ad-26ea93fecd48	1
27	1	8c0520fa-276b-4d21-b4a9-dce1fce59f6b	1
28	1	84adce5c-39c7-425e-b163-4a1a3977364b	1
29	1	91e71279-c9d2-4873-916a-59da03a65741	1
30	1	77acdc13-e1b5-4a6c-9be1-b987e8256f10	1
31	1	aab55af7-8084-47b6-910f-75a75a6313a4	1
32	1	270d14b2-07bc-46bc-918f-658102265ccf	1
33	1	92dfeeb2-1117-422b-87ea-08a589f1134b	1
34	1	661b2159-4040-4a44-b182-46085416ebd9	1
35	1	46665089-aa3d-44c3-964d-6638dfbb5782	1
36	1	0f730bd9-2060-46b1-9208-0ac6562e8b2a	1
37	1	11e02134-7b1a-46a4-a89e-7539dd1efada	1
38	1	3407fe41-fdd3-4119-8f70-4bc4590a379f	1
39	1	5602c14d-cc22-47d7-a1b0-6491749f00dd	1
40	1	d64b0848-0193-4025-ba62-63ecd8fb9f50	1
41	1	5a73d439-bd78-42e1-90ae-eedd30536881	1
42	1	b828251c-86a9-454f-9852-d0876d0f5153	1
43	1	318f7f70-e374-40ef-8afb-3389c10461d8	1
44	1	132ca99a-a3c7-4ed6-b4d0-0edcd7140ca2	1
45	1	1b388371-f9ef-45b4-82a3-ca20a8cd7807	1
46	1	22a23706-a5fe-46f3-b845-6f58d558c723	1
47	1	713332c1-5bd8-400f-bfff-c1ca0697a043	1
48	1	05878e49-93ad-4144-9c50-a0bb86126c2e	1
49	1	51acd176-e84d-42c2-acb2-cceeb28a9fec	1
50	1	41960d32-ddb5-42be-94b2-3a2e77ca148d	1
51	1	0b718d21-6873-4613-ad15-660d30527fcd	1
52	1	70e58068-90d2-4720-a11c-5d243046b4a3	1
53	1	09071f5f-c9cb-43a9-b41d-faf6caf3beff	1
54	1	ae7604bb-4818-45a3-960c-cf3d83f15964	1
55	1	57c9ff89-dd30-467b-bb3e-499eeea8cb94	1
56	1	cdf41cf4-4e77-453d-be5b-0abbbd358934	1
57	1	81763d7d-3897-4be9-bbf6-f6f5dee366ff	1
58	1	f8f4fc60-725d-46d8-8e8f-e68e00d20589	1
59	1	027dd013-baa7-4111-b3c9-f4d1414e9c45	1
60	1	f1750962-a87c-49f6-b731-02ae971ac6ea	1
61	1	89f43e27-790b-4ca1-8ba7-0882b31e0783	1
62	1	5d641bf6-0f93-4189-8dc1-ec7ea446dade	1
63	1	ad1712d8-809f-410c-8b91-ffe6fb8a69a1	1
64	1	17b60106-a4c7-410a-8ac3-ec8e74e29a7c	1
65	1	2d9663be-c466-4191-85e2-a69ce0965432	1
66	1	808a8491-2966-4388-8590-a46ba147f65a	1
67	1	f28b21a6-f7ce-437a-8c5b-0423cb55cefb	1
68	1	c8909015-ea49-47cb-8d37-653904f965bb	1
69	1	e938ee4c-d5df-4d93-bd61-9e518fb1dc30	1
70	1	0e4150db-ac43-48b4-9791-0d874906acf5	1
71	1	d371031e-78be-4964-b521-b1eaa22a5480	1
72	1	8b566757-e5e4-4d47-b5ef-80ed7d853679	1
73	1	8a7643e4-b824-441a-bbe6-349fc7a3e11e	1
74	1	ba0082fb-2d4c-489e-8140-93a6fa693fd0	1
75	1	8b27326f-e7b8-4a4d-b589-df459246d19a	1
76	1	48e603a2-b965-4fbc-ad57-4388bce5ac8b	1
77	1	38630f89-bc22-49b2-b97e-d31a5169decc	1
78	1	3bc757c1-3adb-4321-8832-8e1cc9e687f7	1
79	1	19053c47-97ce-4a25-a92c-1f4061e85bf3	1
80	2	70973a2d-66e6-4662-ab10-99678775dda1	1
81	2	91e71279-c9d2-4873-916a-59da03a65741	1
82	2	1af63a5e-2bec-4f8d-a373-d9ce43a7d242	1
83	2	48e603a2-b965-4fbc-ad57-4388bce5ac8b	1
84	2	28a52ba1-95da-44e1-8ac5-0dc23c902394	1
85	2	bd13392e-e72a-4dbb-ba9d-68f68e47e30e	1
86	2	3b7b862a-7edd-4e96-b4b9-f7113b4c93f9	1
87	2	340d6218-d351-4b2e-80da-7d4afa77190a	1
88	2	c916b81f-700e-4a1a-8c8e-c4685ceaecd2	1
89	2	9895a33f-9bbd-4440-8c1a-0d401431b77f	1
90	2	ef2a24f5-ce5e-4054-843a-2cae0c66318a	1
91	2	2cc439e8-d112-44e6-bc5a-6e99333c519a	1
92	2	3bb518ff-399b-4ce7-b9ad-a1d563dd7792	1
93	2	6ff4ff67-ad08-447f-a112-1a071c1474a4	1
94	2	813c19f5-3580-488d-9eee-c7a563def532	1
95	2	b25bea73-7444-4ee2-8022-611461506117	1
96	2	ab28a801-6ab1-4db2-8ae2-c24bb4c10ebb	1
97	2	d9167046-7908-4d16-8cc1-3eb02d1ba547	1
98	2	5f5e0b10-c8cf-450c-bfd3-bcb0528ec330	1
99	2	c8909015-ea49-47cb-8d37-653904f965bb	1
100	2	55746bf4-86e0-489b-b9dc-eba5f5b401dd	1
101	2	426a5f3f-f161-49b3-97d0-02cda554752b	1
102	2	c15bb7eb-aaaa-4468-9641-8f706d6137e8	1
103	2	808a8491-2966-4388-8590-a46ba147f65a	1
104	2	e938ee4c-d5df-4d93-bd61-9e518fb1dc30	1
105	2	5b2364d7-a811-4595-a1b4-224c70555ffa	1
106	2	ae198ce9-097b-4204-b599-12bdb36f5195	1
107	2	bc71ebf6-2056-41f7-be35-b2e5c34afa99	25
108	2	682cbf2a-2723-4590-ae52-7c5d8b2573cf	1
109	2	f7ff0046-8365-4ea4-9872-583d140f7b3f	1
110	2	04fbb7aa-0dd4-4ac6-837d-822227fb6356	1
111	2	3a1efa7e-21d5-47a1-bcd8-7d4616be0d58	1
112	2	438ee544-78a2-4e32-9343-70838b60ac16	1
113	2	0b6db929-1b6a-4372-8146-aa047258b552	1
114	2	ef027846-be81-4959-a6b5-56bd01b1e68a	1
115	2	91c380f6-2c41-48b7-9f07-104e8659e397	1
116	2	0b49dbdc-4f1d-4895-b263-a48c7b7e6b44	1
117	2	056b5d53-f2fd-4e83-9fd3-d8f0d0be64b9	1
118	2	d89bbfe7-e7ef-4f1c-a6b5-6d8ef4079daa	1
119	2	1e3c1920-7bf5-44f9-b1ff-b17d6420e69b	1
120	2	3d21f710-6bbc-41ae-a8e5-02debe3e02bd	1
121	2	e9f410c1-d93d-4291-95fc-513832fb36e0	1
122	2	93e4308e-e701-440e-861a-a30324654095	1
123	2	ea27c8bf-33c5-443f-b8b2-51235efc2491	1
124	2	63c04040-e109-494e-baa6-c639a6c9a996	1
125	2	18729459-8669-4c56-a78d-fd5982c3b2ac	1
126	2	c2dd6c78-e482-4a3c-ad9c-7dd71b3e34d2	1
127	2	0b55eac6-a745-4bf4-8926-5ce83bc38d7d	1
128	2	f7d8b91b-6541-4d3e-af51-7e000eac69c1	1
129	2	03c35670-84ac-4825-b395-cefb76ceb89a	1
130	2	c3436d70-230c-4bda-b7da-935a270f135f	1
131	2	97435e03-2fff-4fe0-8fa9-69ab0a046a33	1
132	2	8f69a103-d557-4d28-847e-aa94a440e908	1
133	2	f8f4fc60-725d-46d8-8e8f-e68e00d20589	1
134	2	23d349a0-e441-40b8-b634-13e61440a7c8	1
135	2	7b51a979-47e0-4b98-8ec5-8064c83422e1	1
136	2	c40bc9f6-2b72-4ae1-b912-50075af59628	1
137	2	6d611800-5b0e-40ca-82a2-5b294a1ebf6d	1
138	2	b9243163-b726-432e-830f-86132aa7f34a	1
139	2	d371031e-78be-4964-b521-b1eaa22a5480	1
140	2	b9f4f96b-6e54-4fe6-8df7-623e0fc72409	1
141	2	edb6ae9e-a41c-4d82-97bf-4be00e98364c	1
142	2	2ac93092-1a57-4cf7-8f03-1dca86d5c476	1
143	2	a690c1de-158a-4f33-a916-ed2a130f92eb	1
144	2	15d38195-b661-42c1-82c9-1af2a3ba2ea5	1
145	2	b2eb7a64-a307-4a78-a25d-63fb3ae1e237	1
146	2	52232aad-e4ca-4551-8c8d-c69a3b6b49c7	1
147	2	78124cd1-2658-43f3-a339-0755d84154d2	1
148	2	10b8d4c7-7553-4d76-b643-d98b80701e13	1
149	2	f28b21a6-f7ce-437a-8c5b-0423cb55cefb	1
150	2	0b95c115-6745-4ce1-896b-9021e93c161e	1
151	2	4c2196e5-6ed7-4cd2-b8e7-f01aff236569	1
152	2	393a254f-be31-431a-9341-a51286f8cbce	1
153	2	9b7f1d05-707c-4ed3-9f0e-8ced1232c2ee	1
154	2	40c88392-2aa7-4e12-86a4-8d680219e670	1
155	2	8dec6fcf-1254-4b1b-ba23-7a3e492a7241	1
156	3	8747ca90-ac1d-45fd-89fe-aac385cd66b4	1
157	3	00187de2-bc48-4137-97d8-a9a0fafc76c1	1
158	3	5cf069cf-3a10-4427-a02b-5405429b5652	1
159	3	1720d38a-c3c1-4294-85c1-a91fc7179d6f	1
160	3	8c0520fa-276b-4d21-b4a9-dce1fce59f6b	1
161	3	394c6de5-7957-4a0b-a6b9-ee0c707cd022	1
162	3	6a6c5e17-6465-4a1f-9d63-8a3ce2edc522	1
163	3	2f990b54-fbf3-4949-85bb-9ba39710e72a	1
164	3	11cf103c-f8a3-4892-942d-2c4472dd5716	1
165	3	6c0e22f2-f0f3-43e6-87c5-c543032112d8	1
166	3	713332c1-5bd8-400f-bfff-c1ca0697a043	1
167	3	58ea9b0a-1bef-4ccb-acb7-f03c8daf3df3	1
168	3	765863c8-1be0-4bb1-9e9c-db7701cffde3	1
169	3	256d884f-5dea-4906-a53d-cf14d43e6bc8	1
170	3	d64b0848-0193-4025-ba62-63ecd8fb9f50	1
171	3	837182db-1bf3-4a2c-bd01-1af9d9873561	1
172	3	be5d6779-ecc9-44b9-8c4d-97a243059d55	1
173	3	51acd176-e84d-42c2-acb2-cceeb28a9fec	1
174	3	916cb70f-3b06-48ed-972d-75f805aa0892	1
175	3	05878e49-93ad-4144-9c50-a0bb86126c2e	1
176	3	11e02134-7b1a-46a4-a89e-7539dd1efada	1
177	3	46665089-aa3d-44c3-964d-6638dfbb5782	1
178	3	c3a68018-9eff-47e6-a612-182886d28fe3	1
179	3	3407fe41-fdd3-4119-8f70-4bc4590a379f	1
180	3	fa1b722a-109e-4a43-bd7b-818292728cb3	1
181	3	fd51dce2-8e8a-4686-910e-1bbc2825d190	1
182	3	f74c4d96-bc4a-4d32-9519-a753d192144e	1
183	3	bf75a3d1-f184-4b48-a913-21caee1db084	1
184	3	c7b044c3-3cfa-407e-bf20-2875e8e04b7b	1
185	3	5a73d439-bd78-42e1-90ae-eedd30536881	1
186	3	48390b8d-db41-48a5-9bac-316660ab7252	1
187	3	c2c60973-b4d5-43fa-a0c4-b5688b28df05	1
188	3	0b718d21-6873-4613-ad15-660d30527fcd	1
189	3	973fd4d4-9255-4825-85b7-503606c4e932	1
190	3	41d11144-32fc-4e45-a8db-3edb9dc0ce80	1
191	3	81763d7d-3897-4be9-bbf6-f6f5dee366ff	1
192	3	a3fb7228-e76b-4e96-a40e-20b5fed75685	10
193	3	b2c6aa39-2d2a-459c-a555-fb48ba993373	10
194	3	f7c89d03-606e-4284-90dd-64b1fbebfb4d	1
195	3	318f7f70-e374-40ef-8afb-3389c10461d8	1
196	3	9acf2701-2ba1-46fa-ad4a-b130a58181d3	1
197	3	2f4ad084-2062-44c0-9975-15f100204531	1
198	3	96e7e4c8-f66d-4327-aee6-85da648418f5	1
199	3	854fd120-9a51-4d37-9922-7e4b0464e0f5	1
200	3	a9d288b8-cdc1-4e55-a0c9-d6edfc95e65d	1
201	3	7340199d-60c7-4c27-9a0f-047790273db0	1
202	3	d2db3f9c-26fe-487b-bbb7-8bc6f33456f1	1
203	3	41c15160-ee8c-42e3-bc6c-593b8c8ae335	1
204	3	4c05b382-58ab-4a2d-a81c-408ea273b6b6	1
205	3	b5e51171-2e36-4b09-b43d-e5d437a19305	1
206	3	0c85b8f7-0bd0-4680-9ec5-d4b110460a54	1
207	3	f34b9bc4-7bfe-47fd-ba23-4eeeb46026eb	1
208	3	fa71db44-5181-4c51-8b24-7fbedf36e3ca	1
209	3	eac8c196-8477-4b79-9875-21afa1e61708	1
210	3	d14f313c-fea6-49c4-8197-5b74ee584a6b	1
211	3	8f1c1bd6-8420-4de2-81f4-5faa419da301	1
212	3	1cb0610b-a731-42c2-b93f-0a29f63cebf4	1
213	3	f307b5b4-e949-4f69-8dc7-856e33a45a16	1
214	3	27e0948b-9916-473b-8d8c-a51bdfbc7457	1
215	3	c643365a-4255-4d23-adb9-0f8b456f0838	1
216	3	5abcc262-fbe1-4e81-897c-da5a8fb1f1f4	1
217	3	c43b616e-ef87-4c12-810b-a1a3260168fa	1
218	3	a80ea391-54d1-4b61-95a3-f8deb4cbe842	1
219	3	91fbb25b-8521-483f-88b0-77778d25f7fd	1
220	3	cdf41cf4-4e77-453d-be5b-0abbbd358934	1
221	3	5eff8a06-e0d6-435a-a7c0-db9f9d98636a	1
222	3	d6fafb50-9531-4fbf-bb1e-ebb4dd39281c	1
223	3	79f94050-d850-41ca-b1db-5ae0cf743f0a	1
224	3	17039058-822d-409f-938c-b727a366ba63	1
225	3	edac6c23-cff5-4ee2-9227-756b1dc6ab0c	1
226	3	b4ca6a20-b1ba-4dee-ae8a-83f8edf360c4	1
227	3	a9d8ab76-70a4-475e-b87e-4737c090553a	1
228	3	884686dd-515c-4484-a324-e7d126903a42	1
229	3	62007675-8a0d-4211-83b2-0daf3641dedc	1
230	3	01da67a5-e3dc-44f4-8a93-513bfa4d8fbc	1
231	3	158a6225-a246-4fd6-aa57-0df8067b4383	1
232	3	6f981e70-5828-49ac-8a72-cf1cc09be991	1
233	3	17b60106-a4c7-410a-8ac3-ec8e74e29a7c	1
234	3	5e715875-28e0-44b4-af25-f6bb8ddec823	1
235	3	c8b4eba6-88ce-42d7-a235-5a4765a4a981	1
236	3	4ecde8d1-e4ec-4bd9-8a80-49885b032557	1
237	3	e9e56b1d-5ad2-4807-8c4c-1a1a6a25ecbf	1
238	4	2ac93092-1a57-4cf7-8f03-1dca86d5c476	1
239	4	bc71ebf6-2056-41f7-be35-b2e5c34afa99	11
240	4	ea27c8bf-33c5-443f-b8b2-51235efc2491	1
241	4	b9243163-b726-432e-830f-86132aa7f34a	1
242	4	48a99dce-0aa9-4aac-81df-cec5f94c639d	1
243	4	a0c47ab6-dfb4-46ee-a3f7-9e1521b4bb4b	1
244	4	16f93781-6740-40c7-a727-7911beac4e74	1
245	4	592c91fc-6430-4c76-9460-65f047350f67	1
246	4	973fd4d4-9255-4825-85b7-503606c4e932	1
247	4	a8fdbcdf-479d-4582-9ad5-9fbd4c740c29	1
248	4	b2c6aa39-2d2a-459c-a555-fb48ba993373	12
249	4	59c7496b-19a2-478b-b28f-6f153c2458ae	1
250	4	00187de2-bc48-4137-97d8-a9a0fafc76c1	1
251	4	2f990b54-fbf3-4949-85bb-9ba39710e72a	1
252	4	1f9efad1-9f51-47aa-a5a7-9c8103435a01	1
253	4	319d5cf8-ab72-401e-b04e-c13a7a6c1aac	1
254	4	c7b044c3-3cfa-407e-bf20-2875e8e04b7b	1
255	4	1e825298-7c99-40cc-81c9-280cc7ed98d3	1
256	4	c623aeb1-e6d4-48fe-bd2a-a7a6729aa4df	1
257	4	7cfc14ce-9940-4c61-9daa-2c6dbf1a80ad	1
258	4	41d11144-32fc-4e45-a8db-3edb9dc0ce80	1
259	4	ab5ebae2-cd77-4a7d-a93b-8042cd486429	1
260	4	0c85b8f7-0bd0-4680-9ec5-d4b110460a54	1
261	4	a75445d3-1303-4bb5-89ad-26ea93fecd48	1
262	4	8c0520fa-276b-4d21-b4a9-dce1fce59f6b	1
263	4	84adce5c-39c7-425e-b163-4a1a3977364b	1
264	4	77acdc13-e1b5-4a6c-9be1-b987e8256f10	1
265	4	aab55af7-8084-47b6-910f-75a75a6313a4	1
266	4	270d14b2-07bc-46bc-918f-658102265ccf	1
267	4	92dfeeb2-1117-422b-87ea-08a589f1134b	1
268	4	661b2159-4040-4a44-b182-46085416ebd9	1
269	4	46665089-aa3d-44c3-964d-6638dfbb5782	1
270	4	0f730bd9-2060-46b1-9208-0ac6562e8b2a	1
271	4	11e02134-7b1a-46a4-a89e-7539dd1efada	1
272	4	3407fe41-fdd3-4119-8f70-4bc4590a379f	1
273	4	5602c14d-cc22-47d7-a1b0-6491749f00dd	1
274	4	d64b0848-0193-4025-ba62-63ecd8fb9f50	1
275	4	5a73d439-bd78-42e1-90ae-eedd30536881	1
276	4	a5d02339-9f6d-486e-87d8-5ec2274cf680	1
277	4	132ca99a-a3c7-4ed6-b4d0-0edcd7140ca2	1
278	4	1b388371-f9ef-45b4-82a3-ca20a8cd7807	1
279	4	22a23706-a5fe-46f3-b845-6f58d558c723	1
280	4	713332c1-5bd8-400f-bfff-c1ca0697a043	1
281	4	05878e49-93ad-4144-9c50-a0bb86126c2e	1
282	4	51acd176-e84d-42c2-acb2-cceeb28a9fec	1
283	4	41960d32-ddb5-42be-94b2-3a2e77ca148d	1
284	4	0b718d21-6873-4613-ad15-660d30527fcd	1
285	4	70e58068-90d2-4720-a11c-5d243046b4a3	1
286	4	09071f5f-c9cb-43a9-b41d-faf6caf3beff	1
287	4	ae7604bb-4818-45a3-960c-cf3d83f15964	1
288	4	57c9ff89-dd30-467b-bb3e-499eeea8cb94	1
289	4	cdf41cf4-4e77-453d-be5b-0abbbd358934	1
290	4	81763d7d-3897-4be9-bbf6-f6f5dee366ff	1
291	4	f8f4fc60-725d-46d8-8e8f-e68e00d20589	1
292	4	027dd013-baa7-4111-b3c9-f4d1414e9c45	1
293	4	f1750962-a87c-49f6-b731-02ae971ac6ea	1
294	4	89f43e27-790b-4ca1-8ba7-0882b31e0783	1
295	4	5d641bf6-0f93-4189-8dc1-ec7ea446dade	1
296	4	ad1712d8-809f-410c-8b91-ffe6fb8a69a1	1
297	4	17b60106-a4c7-410a-8ac3-ec8e74e29a7c	1
298	4	2d9663be-c466-4191-85e2-a69ce0965432	1
299	4	808a8491-2966-4388-8590-a46ba147f65a	1
300	4	f28b21a6-f7ce-437a-8c5b-0423cb55cefb	1
301	4	c8909015-ea49-47cb-8d37-653904f965bb	1
302	4	e938ee4c-d5df-4d93-bd61-9e518fb1dc30	1
303	4	0e4150db-ac43-48b4-9791-0d874906acf5	1
304	4	d371031e-78be-4964-b521-b1eaa22a5480	1
305	4	8b566757-e5e4-4d47-b5ef-80ed7d853679	1
306	4	8a7643e4-b824-441a-bbe6-349fc7a3e11e	1
307	4	ba0082fb-2d4c-489e-8140-93a6fa693fd0	1
308	4	8b27326f-e7b8-4a4d-b589-df459246d19a	1
309	4	48e603a2-b965-4fbc-ad57-4388bce5ac8b	1
310	4	38630f89-bc22-49b2-b97e-d31a5169decc	1
311	4	3bc757c1-3adb-4321-8832-8e1cc9e687f7	1
312	4	19053c47-97ce-4a25-a92c-1f4061e85bf3	1
313	4	0a7d6489-b239-4ce8-aea3-7b5d0e436f70	1
314	4	91c380f6-2c41-48b7-9f07-104e8659e397	1
315	4	ab28a801-6ab1-4db2-8ae2-c24bb4c10ebb	1
316	4	78124cd1-2658-43f3-a339-0755d84154d2	1
317	5	f2f09757-1931-47c0-a5f0-39280445489d	1
318	5	d18a0815-59d3-4667-b52b-9acda741215e	1
319	5	68954295-54e3-4303-a6bc-fc4547a4e3a3	1
320	5	b1793c3b-25d6-4fae-a99d-cfdd2210ca67	1
321	5	6deba6e8-077b-4602-86af-64db766fbbd9	1
322	5	519fccdb-fde4-42cb-b2ca-35fc36cf984a	1
323	5	35b8fa77-4e85-418b-b335-cd1af127075c	1
324	5	5f2be3c2-060a-43e1-b63b-9cd3c78ffcb0	1
325	5	ff3a69a7-a244-4958-9e8b-d153942c1ba6	1
326	5	34428f42-03ac-4795-8286-6cbea796df2b	1
327	5	55eb9310-f24b-410b-850b-a7d0d5117946	1
328	5	e119869d-1f66-402b-8251-d351965cf0d3	1
329	5	d00ade82-caab-43c6-9756-6e9e6edfc519	1
330	5	c8909015-ea49-47cb-8d37-653904f965bb	1
331	5	24b5dbe3-578a-4897-8215-e723f00a8f1a	1
332	5	332b6ed7-90de-46c0-85ad-1ad708b2c1e1	1
333	5	808a8491-2966-4388-8590-a46ba147f65a	1
334	5	7579f57d-c76e-4703-a030-34fb7160cb23	1
335	5	6996c354-d3fa-40c3-8534-27d2f5b74756	1
336	5	bdbde5d0-f5e4-44da-b27c-b4ad6f374cc9	1
337	5	1eb4c001-1115-4b99-bbbf-703eb551a381	1
338	5	71d14755-b81d-4aec-b94d-1637884a5844	1
339	5	0b5d174c-7d4d-4c31-a582-8169d3bc932a	1
340	5	ea9c459a-6047-43aa-968f-a582be4000e8	1
341	5	5da7eea8-bb9e-47ce-a554-8a1ee058bd7a	1
342	5	146eb8c8-93d5-4dd5-af7a-46d5acd4a89d	1
343	5	d7337b13-5459-434a-aa56-e1547b875594	1
344	5	642bbdf6-9a8c-4158-83c8-d59feb49101b	1
345	5	db4907f3-4071-47c4-9329-96b4813435be	1
346	5	4c6c8155-efa0-4c22-b255-34974a80c496	1
347	5	b685757b-521e-4353-a233-97052359723d	1
348	5	5aa3abf1-d56b-4f42-8c84-7e5a2c15ee0f	1
349	5	2b76f9e9-cd28-4eaf-8674-215c34263f96	1
350	5	7256b03e-4fd5-4a0a-9b20-7b88870edfec	1
351	5	b5f20a6d-8c3e-452f-9d98-4886f0fa052a	1
352	5	8c411f4e-a091-447c-9450-d895b10b4985	1
353	5	22c75ad5-237c-4dbd-bbc4-3058a3db408f	1
354	5	8b610f8f-c8dd-4eeb-bc6e-3bc706d5f63e	1
355	5	f28b21a6-f7ce-437a-8c5b-0423cb55cefb	1
356	5	78427103-9543-41fb-b6d4-72963fe87275	1
357	5	69b52907-8aff-4f2a-a391-7d1ab2669b5c	1
358	5	69872a9a-fe54-4e58-940c-89395af71acd	1
359	5	ff8f5a4b-112a-425e-b489-7ee26d1d9fb3	1
360	5	402ec768-76fb-474e-ae74-babc90d833c4	1
361	5	70a6f08e-854d-4e2f-9d8c-c45ec3231157	1
362	5	8dcb35e5-ae44-455f-86e3-4a77d496ff34	1
363	5	3cef501b-5ce1-4699-89a5-3949f7e65bf1	1
364	5	0c26ab0d-80f6-4e5b-9d0e-af17c1519583	1
365	5	34bcc217-dd91-45a0-90d7-a94d02f1f317	1
366	5	1efe485e-6874-4f4c-b981-e10f41670865	1
367	5	013e4281-4e3e-47c2-ac9a-f570668bcb16	1
368	5	60c60923-ff1b-43f7-8768-731499fcffc9	1
369	5	ee302659-59ed-4eef-babe-451b9ccf7f14	1
370	5	de83b309-ac31-4a57-95e6-ccd3659e6c1f	1
371	5	b543bfa2-0174-4dd8-9ee6-09b92fb44934	1
372	5	6bd872b2-5c40-4e11-9a7f-0136a51b0642	1
373	5	0715e860-3b3b-4331-9718-207973e94fee	1
374	5	ebf7ce9b-9e5e-4557-9e28-76556997f0ee	1
375	5	d209b948-9afb-4fd1-a961-72c87282878c	1
376	5	a0c47ab6-dfb4-46ee-a3f7-9e1521b4bb4b	1
377	5	40c88392-2aa7-4e12-86a4-8d680219e670	1
378	5	f33ce38a-34ec-4b65-a0fc-160484a02007	1
379	5	a75445d3-1303-4bb5-89ad-26ea93fecd48	1
380	5	0c85b8f7-0bd0-4680-9ec5-d4b110460a54	1
381	5	aa959340-c869-4caa-92c7-572bd8d23eef	1
382	5	134d5b82-7940-4b33-a922-7f9d1f403e50	1
383	5	96b909ab-55f1-4cde-a675-2504de3da772	1
384	5	45429b2c-be3b-4b2e-9bab-a059ccbda8cd	1
385	5	f413a83d-a40d-434c-b20a-4c707c0527fa	1
386	5	e521322b-0e83-458c-8936-7021a80ee279	1
387	5	7e26f0b7-20e6-46d5-8130-d98c14d6aa29	1
388	5	20283c4a-f1f0-42f0-bc08-6da87474426b	1
389	5	fb5a3403-7f0b-406c-8c4f-d693be010ca6	1
390	5	ec96cde2-f1e6-495c-94e2-3e8ae79e556c	1
391	5	bc71ebf6-2056-41f7-be35-b2e5c34afa99	4
392	5	b2c6aa39-2d2a-459c-a555-fb48ba993373	4
393	5	b34bb2dc-c1af-4d77-b0b3-a0fb342a5fc6	15
394	5	78124cd1-2658-43f3-a339-0755d84154d2	1
395	5	2a2ea189-b663-4f4a-bb23-ff7a4af25f71	1
396	5	c52cfb41-18f3-4e73-b5e7-d75baf74e578	1
397	6	f825c98f-a327-440b-8c0d-ebe02e23bfb7	1
398	6	55eb9310-f24b-410b-850b-a7d0d5117946	1
399	6	34428f42-03ac-4795-8286-6cbea796df2b	1
400	6	f2f09757-1931-47c0-a5f0-39280445489d	1
401	6	5e958212-6a5b-4288-8d31-f1572619d7dc	1
402	6	032ec6e2-6cc3-4a97-9cc7-3233f5e11904	1
403	6	27c2b8ce-17b9-4f6c-8444-3bdc636dd413	1
404	6	e119869d-1f66-402b-8251-d351965cf0d3	1
405	6	b8619990-9dc2-4fcc-bc7e-457b77cd2a8e	1
406	6	69b52907-8aff-4f2a-a391-7d1ab2669b5c	1
407	6	a494cbe7-a4a5-4657-9f68-3811fe56a17e	1
408	6	2ad8ff62-d090-4835-9274-3b755ba0f8e6	1
409	6	f60b879e-82b4-442d-a544-2349429fe4c9	1
410	6	2db17cfc-0947-407f-bd53-d64a48eec314	1
411	6	ed5d6aba-0b8d-48b3-a84c-618f45fb67b1	1
412	6	3b4eaa50-dff6-4825-97da-42b2f5554218	1
413	6	1080c5b5-6651-4c6a-93e6-099fbe389e26	1
414	6	55cec4bc-3de6-4c0d-a4cc-c5a30849fbda	1
415	6	7e84aff5-2cb6-4214-befd-3d2de31229e5	1
416	6	146eb8c8-93d5-4dd5-af7a-46d5acd4a89d	1
417	6	5c6c8fe7-3520-423b-a224-1c0af516871a	1
418	6	5f2be3c2-060a-43e1-b63b-9cd3c78ffcb0	1
419	6	a5e54d2b-aad8-4ddd-af4b-13668913762b	1
420	6	3adbd963-e85d-4569-963a-4472594f06f9	1
421	6	efe9c3e0-7315-408e-9903-9d25a681039b	1
422	6	2f807301-37df-4724-871a-08e3512b07b3	1
423	6	159098f1-172f-408e-a962-86763a29a8e1	1
424	6	793b0c73-601c-41dc-b49a-45fee4970d52	1
425	6	8ca3852a-fa37-46ef-a74d-3c7487ae50fc	1
426	6	78427103-9543-41fb-b6d4-72963fe87275	1
427	6	22c75ad5-237c-4dbd-bbc4-3058a3db408f	1
428	6	6cac6f76-f3c6-4b00-a468-166b07853401	1
429	6	b685757b-521e-4353-a233-97052359723d	1
430	6	4f9e07ae-6341-4b46-9f77-f17ab659d266	1
431	6	3875aef0-3102-4fbf-be90-e4139f7a2348	1
432	6	9fb8cd81-403a-4988-8f1c-b8eccf8abd9c	1
433	6	58934f6d-1aa2-414c-85c6-955a1e26d675	1
434	6	649e7237-b38b-43e9-83f4-763751fb1bea	1
435	6	0c85b8f7-0bd0-4680-9ec5-d4b110460a54	1
436	6	5da7eea8-bb9e-47ce-a554-8a1ee058bd7a	1
437	6	60bc63dc-ac9f-4a2f-aef5-c90d0aa31553	1
438	6	0b5d174c-7d4d-4c31-a582-8169d3bc932a	1
439	6	b1793c3b-25d6-4fae-a99d-cfdd2210ca67	1
440	6	56719f6a-1a6c-4c0a-8d21-18f7d7350b68	10
441	6	68954295-54e3-4303-a6bc-fc4547a4e3a3	1
442	6	44ed4c0c-a012-4895-a547-b04150553bba	1
443	6	b5f20a6d-8c3e-452f-9d98-4886f0fa052a	1
444	6	38ddbe0b-07ef-4cdf-ad6c-cd8b8ba6d206	1
445	6	2e3ee458-fa3f-4452-ab95-5a7fb5a0483b	1
446	6	fdd46004-eaba-4024-8687-39b23dc6a58c	1
447	6	e8c7566d-7cc0-48af-a986-83223ec7e06c	1
448	6	8aa1bbc9-b047-4d68-93f4-728e40dafc5e	1
449	6	ea2dce18-195e-4681-a687-f9819edaf9fc	1
450	6	cdaab6b0-1a2d-4809-8e6b-56013acd8f78	1
451	6	b34bb2dc-c1af-4d77-b0b3-a0fb342a5fc6	10
452	6	7b459306-149b-4f43-abc1-2dd70c748c0e	1
453	6	4b8bf64b-4800-45ff-81c6-2857f34999b5	1
454	6	df7d4964-35a2-42f7-a4f5-05122f78cbba	1
455	6	e5456b35-d7a9-4717-90df-c6c46f1ec437	1
456	6	dc55421f-dee8-4263-9df0-2365df5f14bb	1
457	6	6de714e1-446d-4fb9-9e3d-bcd3ec6af9ca	1
458	6	326ba371-124c-4949-a048-3a0c8962e567	1
459	6	de75e5dd-8a52-406c-b55c-96d686885500	1
460	6	be811e70-aaaa-41f3-bf9e-5d3f9f719b49	1
461	6	060209fa-4b7d-4e08-bf8b-8759bba5641b	1
462	6	131096a3-2d53-4ec2-96d3-fcb58e698a13	1
463	6	a75445d3-1303-4bb5-89ad-26ea93fecd48	1
464	6	3bb518ff-399b-4ce7-b9ad-a1d563dd7792	1
465	6	71d14755-b81d-4aec-b94d-1637884a5844	1
466	6	24b5dbe3-578a-4897-8215-e723f00a8f1a	1
467	6	c9fe1383-1331-4a58-a45a-3320250221a9	1
468	6	975ec9a3-6f20-4177-8211-82526e092538	1
469	6	fa2da325-6859-45bb-b185-35526b01bcc1	1
470	6	ad6a2776-801f-4743-8268-6d654122171e	1
471	6	9f8fe514-77ed-41b4-a6f3-c6f095bb97be	1
472	6	ec2b3779-55f7-4169-aa66-6312fb52721f	1
473	6	aa959340-c869-4caa-92c7-572bd8d23eef	1
474	6	7256b03e-4fd5-4a0a-9b20-7b88870edfec	1
475	6	8b21062e-97b0-4967-96df-30e8309f4fba	1
476	6	25eaa977-ca85-4f8d-8be5-9297a3edb14f	1
477	6	50863075-a30b-4238-8e31-5af32a39d886	1
478	6	43b8456a-3333-4936-a09c-324327619c36	1
479	7	326ba371-124c-4949-a048-3a0c8962e567	1
480	7	6cac6f76-f3c6-4b00-a468-166b07853401	1
481	7	f8f4fc60-725d-46d8-8e8f-e68e00d20589	1
482	7	8c411f4e-a091-447c-9450-d895b10b4985	1
483	7	5f2be3c2-060a-43e1-b63b-9cd3c78ffcb0	1
484	7	a3afd8b9-d499-40f7-be41-f8fee6636a1d	1
485	7	c2dd6c78-e482-4a3c-ad9c-7dd71b3e34d2	1
486	7	3b03358d-f87e-4939-afc9-5ee3f044146a	1
487	7	f6ed46e2-08f9-4e89-aae3-e5dd7f30f007	1
488	7	4f9e07ae-6341-4b46-9f77-f17ab659d266	1
489	7	746dc1d8-f245-47be-abd5-50ca6ac0d1af	1
490	7	9f8fe514-77ed-41b4-a6f3-c6f095bb97be	1
491	7	6deba6e8-077b-4602-86af-64db766fbbd9	1
492	7	e8c7566d-7cc0-48af-a986-83223ec7e06c	1
493	7	b8ee0dfc-f588-43b6-ab96-f3a3c58a8a3d	1
494	7	44ed4c0c-a012-4895-a547-b04150553bba	1
495	7	8dec6fcf-1254-4b1b-ba23-7a3e492a7241	1
496	7	7e84aff5-2cb6-4214-befd-3d2de31229e5	1
497	7	befb211f-37ca-4083-98d4-9ff1f28be3f2	1
498	7	183891b0-b5ec-47f4-8d09-b9d3cfc4e7f1	1
499	7	acdbaa99-18c9-4aa1-8797-69837ca7d8f5	1
500	7	a1d17244-9c92-4094-9843-d7ee31c85ea4	1
501	7	3113afec-d4ae-46b5-9952-89e2e2c9ae7b	1
502	7	91e71279-c9d2-4873-916a-59da03a65741	1
503	7	5674f6ae-ed5d-441e-a534-b5dd415165fd	1
504	7	97435e03-2fff-4fe0-8fa9-69ab0a046a33	1
505	7	4a01db2e-cd43-4b1a-a480-169018f82501	1
506	7	d00ade82-caab-43c6-9756-6e9e6edfc519	1
507	7	34428f42-03ac-4795-8286-6cbea796df2b	1
508	7	b1793c3b-25d6-4fae-a99d-cfdd2210ca67	1
509	7	6a290adf-7b75-451e-bb79-9c13e10b38aa	1
510	7	40c88392-2aa7-4e12-86a4-8d680219e670	1
511	7	258a78cc-a9f3-459a-9837-93a9856f72db	1
512	7	808a8491-2966-4388-8590-a46ba147f65a	1
513	7	212af747-2ac1-4d33-9456-8b3cc8988f5d	1
514	7	4d67357c-5939-45c8-a656-aaf76728a5a6	1
515	7	55cec4bc-3de6-4c0d-a4cc-c5a30849fbda	1
516	7	a0c47ab6-dfb4-46ee-a3f7-9e1521b4bb4b	1
517	7	7af0e2da-9163-42d9-bf69-439cc61cd28d	1
518	7	55eb9310-f24b-410b-850b-a7d0d5117946	1
519	7	2348f3d4-02cc-4611-a7e0-b77604d5f2e8	1
520	7	69b52907-8aff-4f2a-a391-7d1ab2669b5c	1
521	7	b0b6be0c-41cf-4757-9f0e-87227b6ba6b3	1
522	7	57b37df5-fee4-4720-931f-f0cb0a8b338c	1
523	7	faa01ed1-ccfa-4e58-951f-cd81f9068027	1
524	7	fa2da325-6859-45bb-b185-35526b01bcc1	1
525	7	4a3acbf7-0699-4d7c-b4fa-aa15c7cde189	1
526	7	ac10d218-f9a6-4058-9cda-a15ca1b0b7b5	1
527	7	793b0c73-601c-41dc-b49a-45fee4970d52	1
528	7	63e6cb7d-cc27-4200-85fc-ff6472318c1a	1
529	7	70a6f08e-854d-4e2f-9d8c-c45ec3231157	1
530	7	c4212ba4-8c43-40ca-aa36-b3b535659fa8	1
531	7	68954295-54e3-4303-a6bc-fc4547a4e3a3	1
532	7	002965be-a36f-4a09-9ce0-c6535bca1703	1
533	7	667ece0e-a474-4e11-8ac4-6c8fc354457c	1
534	7	7e5d9efe-48a9-434b-bb09-056e0e09cc9a	1
535	7	73864fcc-1bde-4bc0-831e-2b93e546e417	1
536	7	d37f858e-03c8-4594-9b92-cd03699a1591	1
537	7	e6e6fce8-0f6a-4b84-865e-d4e4a4182f9f	1
538	7	6de714e1-446d-4fb9-9e3d-bcd3ec6af9ca	1
539	7	dc55421f-dee8-4263-9df0-2365df5f14bb	1
540	7	975ec9a3-6f20-4177-8211-82526e092538	1
541	7	c9fe1383-1331-4a58-a45a-3320250221a9	1
542	7	f413a83d-a40d-434c-b20a-4c707c0527fa	1
543	7	402ec768-76fb-474e-ae74-babc90d833c4	1
544	7	45429b2c-be3b-4b2e-9bab-a059ccbda8cd	1
545	7	0c85b8f7-0bd0-4680-9ec5-d4b110460a54	1
546	7	b34bb2dc-c1af-4d77-b0b3-a0fb342a5fc6	7
547	7	56719f6a-1a6c-4c0a-8d21-18f7d7350b68	6
548	7	bc71ebf6-2056-41f7-be35-b2e5c34afa99	5
549	7	cdaab6b0-1a2d-4809-8e6b-56013acd8f78	1
550	7	78124cd1-2658-43f3-a339-0755d84154d2	1
551	7	1ae54fe7-b1d3-4c13-a8ef-f502cf3eb1a0	1
552	7	3641d572-8335-4804-88ad-edf4dc67a8e4	1
553	7	1080c5b5-6651-4c6a-93e6-099fbe389e26	1
554	7	70973a2d-66e6-4662-ab10-99678775dda1	1
555	7	146eb8c8-93d5-4dd5-af7a-46d5acd4a89d	1
556	7	8b21062e-97b0-4967-96df-30e8309f4fba	1
557	7	8b610f8f-c8dd-4eeb-bc6e-3bc706d5f63e	1
558	7	5da7eea8-bb9e-47ce-a554-8a1ee058bd7a	1
559	7	d7337b13-5459-434a-aa56-e1547b875594	1
560	7	cac95494-0db0-4bec-8665-998431a6f76b	1
561	7	5c6c8fe7-3520-423b-a224-1c0af516871a	1
562	7	3c037103-5af0-40a8-bbbe-7f3f5958b6e9	1
563	7	a75445d3-1303-4bb5-89ad-26ea93fecd48	1
564	8	aff34f28-f707-4458-8af3-1bd5b13a6b10	1
565	8	ae7604bb-4818-45a3-960c-cf3d83f15964	1
566	8	2ffc7509-c7ce-44c4-a8cf-90ed161b5d10	1
567	8	7c520355-1ab8-4d6d-9a29-0f63d6b60024	1
568	8	be811e70-aaaa-41f3-bf9e-5d3f9f719b49	1
569	8	a98c2d81-4add-4292-bbdd-e1b69ff936d4	1
570	8	1080c5b5-6651-4c6a-93e6-099fbe389e26	1
571	8	5e958212-6a5b-4288-8d31-f1572619d7dc	1
572	8	ff768016-67f8-409e-8359-9ed05bcb46d2	1
573	8	cdaab6b0-1a2d-4809-8e6b-56013acd8f78	1
574	8	3641d572-8335-4804-88ad-edf4dc67a8e4	1
575	8	36c78a5f-0148-4596-a346-f8e35037b694	1
576	8	ab26fbe2-e808-48b9-8d0d-3fbb6c3d554f	1
577	8	faa01ed1-ccfa-4e58-951f-cd81f9068027	1
578	8	29e9cf1c-a6bd-4bee-9000-ac1b4e19d6b0	1
579	8	bc8de6c7-c69d-4add-8f25-825d945874f9	1
580	8	0f264e5b-264e-4e97-9a8d-8ae1d6a286ce	1
581	8	afb278f1-0e21-4f6d-96bb-c34aa3ef96b0	1
582	8	03c35670-84ac-4825-b395-cefb76ceb89a	1
583	8	d9167046-7908-4d16-8cc1-3eb02d1ba547	1
584	8	808a8491-2966-4388-8590-a46ba147f65a	1
585	8	c8909015-ea49-47cb-8d37-653904f965bb	1
586	8	f28b21a6-f7ce-437a-8c5b-0423cb55cefb	1
587	8	40c88392-2aa7-4e12-86a4-8d680219e670	1
588	8	e938ee4c-d5df-4d93-bd61-9e518fb1dc30	1
589	8	2a2ea189-b663-4f4a-bb23-ff7a4af25f71	1
590	8	1b9a5170-39c0-4cbf-a041-f3c15f1359ae	1
591	8	1267dfda-eb1a-4963-9fe3-fa619d924d7a	1
592	8	bf5f348e-d748-48e2-b483-740109e71176	1
593	8	38ddbe0b-07ef-4cdf-ad6c-cd8b8ba6d206	1
594	8	0e4150db-ac43-48b4-9791-0d874906acf5	1
595	8	bd16434d-55ea-4c5a-a9ef-752971a4af16	1
596	8	68b51d58-7566-4a26-8c31-8da32799bea7	1
597	8	ebc9ff52-c914-4c48-9d35-0e4cd07741cb	1
598	8	c5c2d209-e3ef-4b0d-85f5-e7402dcf09eb	1
599	8	f2f165b6-ef0a-42ad-9352-ba68be8248b0	1
600	8	d37f858e-03c8-4594-9b92-cd03699a1591	1
601	8	1e825298-7c99-40cc-81c9-280cc7ed98d3	1
602	8	5d641bf6-0f93-4189-8dc1-ec7ea446dade	1
603	8	713332c1-5bd8-400f-bfff-c1ca0697a043	1
604	8	58ea9b0a-1bef-4ccb-acb7-f03c8daf3df3	1
605	8	41960d32-ddb5-42be-94b2-3a2e77ca148d	1
606	8	a1d17244-9c92-4094-9843-d7ee31c85ea4	1
607	8	77acdc13-e1b5-4a6c-9be1-b987e8256f10	1
608	8	a2cb30d7-c0a9-4f79-bc19-fc88c4986725	1
609	8	08e2fead-53e9-445b-8fd1-c108bb09d81c	1
610	8	1720d38a-c3c1-4294-85c1-a91fc7179d6f	1
611	8	661b2159-4040-4a44-b182-46085416ebd9	1
612	8	92dfeeb2-1117-422b-87ea-08a589f1134b	1
613	8	8f1c1bd6-8420-4de2-81f4-5faa419da301	1
614	8	84d0c616-2fce-47f4-bfdd-783741a63fd2	1
615	8	57b37df5-fee4-4720-931f-f0cb0a8b338c	1
616	8	5d96840e-f9b3-4c86-8f59-f12e5736318f	1
617	8	326ba371-124c-4949-a048-3a0c8962e567	1
618	8	89f43e27-790b-4ca1-8ba7-0882b31e0783	1
619	8	132ca99a-a3c7-4ed6-b4d0-0edcd7140ca2	1
620	8	1b388371-f9ef-45b4-82a3-ca20a8cd7807	1
621	8	44ed4c0c-a012-4895-a547-b04150553bba	1
622	8	1ae54fe7-b1d3-4c13-a8ef-f502cf3eb1a0	1
623	8	f74c4d96-bc4a-4d32-9519-a753d192144e	1
624	8	70e58068-90d2-4720-a11c-5d243046b4a3	1
625	8	e6e6fce8-0f6a-4b84-865e-d4e4a4182f9f	1
626	8	4bae4e34-fcf4-4aee-8da2-5b3ee41a595a	1
627	8	f1750962-a87c-49f6-b731-02ae971ac6ea	1
628	8	c7b044c3-3cfa-407e-bf20-2875e8e04b7b	1
629	8	027dd013-baa7-4111-b3c9-f4d1414e9c45	1
630	8	7e5d9efe-48a9-434b-bb09-056e0e09cc9a	1
631	8	a5d02339-9f6d-486e-87d8-5ec2274cf680	1
632	8	8b3febb9-0597-46af-b005-982f1a31c83e	1
633	8	8b2b00ab-f1c5-4057-9957-d7daac95a847	1
634	8	52d14717-0cbc-4d7e-b546-54ea91580338	1
635	8	ad1712d8-809f-410c-8b91-ffe6fb8a69a1	1
636	8	81763d7d-3897-4be9-bbf6-f6f5dee366ff	1
637	8	73864fcc-1bde-4bc0-831e-2b93e546e417	1
638	8	b2187f45-80ae-4ac4-9f83-5eb7a00978e2	1
639	8	060209fa-4b7d-4e08-bf8b-8759bba5641b	1
640	8	b2c6aa39-2d2a-459c-a555-fb48ba993373	3
641	8	f7c89d03-606e-4284-90dd-64b1fbebfb4d	1
642	8	56719f6a-1a6c-4c0a-8d21-18f7d7350b68	2
643	8	bc71ebf6-2056-41f7-be35-b2e5c34afa99	3
644	8	f8f4fc60-725d-46d8-8e8f-e68e00d20589	1
645	8	8dec6fcf-1254-4b1b-ba23-7a3e492a7241	1
646	8	cdf41cf4-4e77-453d-be5b-0abbbd358934	1
647	8	765863c8-1be0-4bb1-9e9c-db7701cffde3	1
648	8	17b60106-a4c7-410a-8ac3-ec8e74e29a7c	1
649	8	865a2194-fca0-446e-aae3-ca475cd66e00	1
650	8	819fc966-434e-470f-91e9-a38df974ad17	1
651	8	fc9ec820-4245-4a96-b009-5308a818ca58	1
652	8	33b9b3bd-33ca-46f3-b8bb-a978bc3d1085	1
653	8	ec2b3779-55f7-4169-aa66-6312fb52721f	1
654	8	393a254f-be31-431a-9341-a51286f8cbce	1
655	8	0c85b8f7-0bd0-4680-9ec5-d4b110460a54	1
656	8	f825c98f-a327-440b-8c0d-ebe02e23bfb7	1
657	8	2ec4288e-34c6-4831-a2c0-ba1ca1d9d1dc	1
658	8	c7fa1dda-9312-4ec8-82cd-a1ba7bc33497	1
\.


--
-- Data for Name: matches; Type: TABLE DATA; Schema: public; Owner: -
--

COPY public.matches (id, date, winning_deck, losing_deck, winner_wins, loser_wins, confirmed) FROM stdin;
1	2020-04-26 17:38:21.439297+00	3	4	2	0	t
3	2020-04-26 18:08:10.076114+00	3	2	2	1	t
4	2020-04-26 18:33:34.316675+00	2	5	2	0	t
5	2020-04-26 19:53:10.272809+00	6	5	2	0	t
6	2020-04-26 20:09:15.170225+00	6	4	2	1	t
7	2020-04-26 20:39:39.143454+00	6	2	2	1	t
8	2020-04-26 20:52:36.476062+00	4	7	2	1	t
9	2020-04-27 16:20:45.106788+00	4	2	2	0	t
10	2020-04-27 19:22:16.426558+00	8	4	2	1	t
11	2020-04-27 19:56:23.039166+00	6	8	2	0	t
\.


--
-- Data for Name: disputes; Type: TABLE DATA; Schema: public; Owner: -
--

COPY public.disputes (id, matchid, disputer, date, resolved, note) FROM stdin;
\.


--
-- Name: cards_id_seq; Type: SEQUENCE SET; Schema: public; Owner: -
--

SELECT pg_catalog.setval('public.cards_id_seq', 51430, true);


--
-- Name: deck_contents_id_seq; Type: SEQUENCE SET; Schema: public; Owner: -
--

SELECT pg_catalog.setval('public.deck_contents_id_seq', 658, true);


--
-- Name: deck_view_tokens_id_seq; Type: SEQUENCE SET; Schema: public; Owner: -
--

SELECT pg_catalog.setval('public.deck_view_tokens_id_seq', 28, true);


--
-- Name: decks_id_seq; Type: SEQUENCE SET; Schema: public; Owner: -
--

SELECT pg_catalog.setval('public.decks_id_seq', 8, true);


--
-- Name: disputes_id_seq; Type: SEQUENCE SET; Schema: public; Owner: -
--

SELECT pg_catalog.setval('public.disputes_id_seq', 1, false);


--
-- Name: leagues_id_seq; Type: SEQUENCE SET; Schema: public; Owner: -
--

SELECT pg_catalog.setval('public.leagues_id_seq', 1, true);


--
-- Name: matches_id_seq; Type: SEQUENCE SET; Schema: public; Owner: -
--

SELECT pg_catalog.setval('public.matches_id_seq', 11, true);


--
-- Name: users_id_seq; Type: SEQUENCE SET; Schema: public; Owner: -
--

SELECT pg_catalog.setval('public.users_id_seq', 8, true);


--
-- PostgreSQL database dump complete
--
